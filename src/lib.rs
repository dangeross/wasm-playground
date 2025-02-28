mod errors;
pub mod model;
mod sdk;

use sdk_common::input_parser::parse;
use sdk_common::invoice::parse_invoice;
use std::sync::Arc;
use wasm_bindgen::prelude::*;

use crate::model::*;
use crate::sdk::Sdk;

pub struct WasmEventListener {
    pub listener: EventListener,
}

impl crate::sdk::model::EventListener for WasmEventListener {
    fn on_event(&self, e: crate::sdk::model::SdkEvent) {
        self.listener.on_event(e.into());
    }
}

#[wasm_bindgen(typescript_custom_section)]
const EVENT_INTERFACE: &'static str = r#"export interface EventListener {
    onEvent: (e: SdkEvent) => void
}"#;

#[wasm_bindgen(typescript_custom_section)]
const LOG_INTERFACE: &'static str = "export interface Logger {
    log: (l: LogEntry) => void
}";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "EventListener")]
    pub type EventListener;

    #[wasm_bindgen(structural, method, js_name = onEvent)]
    pub fn on_event(this: &EventListener, e: SdkEvent);

    #[wasm_bindgen(typescript_type = "Logger")]
    pub type Logger;

    /// Handle message
    #[wasm_bindgen(structural, method, js_name = log)]
    pub fn log(this: &Logger, l: LogEntry);
}

#[wasm_bindgen]
pub struct WasmPlayground {
    sdk: Arc<Sdk>,
}

#[wasm_bindgen]
impl WasmPlayground {
    pub fn new() -> WasmResult<WasmPlayground> {
        let sdk = Arc::new(Sdk::new()?);
        Ok(WasmPlayground { sdk })
    }

    #[wasm_bindgen(js_name = "fetchFiatRates")]
    pub async fn fetch_fiat_rates(&self) -> WasmResult<Vec<Rate>> {
        let rates = self.sdk.fetch_fiat_rates().await?;
        Ok(rates.into_iter().map(|r| r.into()).collect())
    }

    #[wasm_bindgen(js_name = "listFiatCurrencies")]
    pub async fn list_fiat_currencies(&self) -> WasmResult<Vec<FiatCurrency>> {
        let currencies = self.sdk.list_fiat_currencies().await?;
        Ok(currencies.into_iter().map(|r| r.into()).collect())
    }

    #[wasm_bindgen(js_name = "parse")]
    pub async fn parse(&self, input: String) -> WasmResult<InputType> {
        let res = parse(&input, None).await?;
        Ok(res.into())
    }

    #[wasm_bindgen(js_name = "parseInvoice")]
    pub fn parse_invoice(&self, bolt11: String) -> WasmResult<LNInvoice> {
        let res = parse_invoice(&bolt11)?;
        Ok(res.into())
    }

    #[wasm_bindgen(js_name = "addEventListener")]
    pub async fn add_event_listener(&self, listener: EventListener) -> WasmResult<String> {
        Ok(self.sdk
            .add_event_listener(Box::new(WasmEventListener { listener }))
            .await?)
    }
}
