mod errors;
pub mod model;
mod sdk;
mod signer;

use std::str::FromStr;

use anyhow::anyhow;
use sdk_common::input_parser::parse;
use sdk_common::invoice::parse_invoice;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use log::{Level, debug};

use crate::model::*;
use crate::sdk::Sdk;
use crate::signer::{Signer, WasmSigner};

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

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(typescript_type = "EventListener")]
    pub type EventListener;

    #[wasm_bindgen(structural, method, js_name = onEvent)]
    pub fn on_event(this: &EventListener, e: SdkEvent);
}

#[wasm_bindgen(js_name = "setLogger")]
pub fn set_logger(level: String) -> WasmResult<()> {
    Ok(console_log::init_with_level(Level::from_str(&level)?)
        .map_err(|_| anyhow!("Logger already created"))?)
}

#[wasm_bindgen]
pub struct WasmPlayground {
    sdk: Arc<Sdk>,
}

#[wasm_bindgen]
impl WasmPlayground {
    #[wasm_bindgen(js_name = "new")]
    pub fn new() -> WasmResult<WasmPlayground> {
        let sdk = Arc::new(Sdk::new()?);
        Ok(WasmPlayground { sdk })
    }

    #[wasm_bindgen(js_name = "newWithSigner")]
    pub fn new_with_signer(signer: Signer) -> WasmResult<WasmPlayground> {
        let wasm_signer = Box::new(WasmSigner { signer });
        let sdk = Arc::new(Sdk::new_with_signer(wasm_signer)?);
        Ok(WasmPlayground { sdk })
    }

    #[wasm_bindgen(js_name = "fetchFiatRates")]
    pub async fn fetch_fiat_rates(&self) -> WasmResult<Vec<Rate>> {
        debug!("fetch_fiat_rates");
        let rates = self.sdk.fetch_fiat_rates().await?;
        Ok(rates.into_iter().map(|r| r.into()).collect())
    }

    #[wasm_bindgen(js_name = "listFiatCurrencies")]
    pub async fn list_fiat_currencies(&self) -> WasmResult<Vec<FiatCurrency>> {
        debug!("list_fiat_currencies");
        let currencies = self.sdk.list_fiat_currencies().await?;
        Ok(currencies.into_iter().map(|r| r.into()).collect())
    }

    #[wasm_bindgen(js_name = "parse")]
    pub async fn parse(&self, input: String) -> WasmResult<InputType> {
        debug!("parse");
        let res = parse(&input, None).await?;
        Ok(res.into())
    }

    #[wasm_bindgen(js_name = "parseInvoice")]
    pub fn parse_invoice(&self, bolt11: String) -> WasmResult<LNInvoice> {
        debug!("parse_invoice");
        let res = parse_invoice(&bolt11)?;
        Ok(res.into())
    }

    #[wasm_bindgen(js_name = "addEventListener")]
    pub async fn add_event_listener(&self, listener: EventListener) -> WasmResult<String> {
        debug!("add_event_listener");
        Ok(self.sdk
            .add_event_listener(Box::new(WasmEventListener { listener }))
            .await?)
    }
}
