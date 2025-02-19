use sdk_common::input_parser::parse;
use sdk_common::prelude::{BreezServer, FiatAPI, InputType, PRODUCTION_BREEZSERVER_URL};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPlayground {
    fiat_api: Arc<dyn FiatAPI>,
}

#[wasm_bindgen]
impl WasmPlayground {
    pub fn new() -> Result<WasmPlayground, JsValue> {
        let breez_server = Arc::new(
            BreezServer::new(PRODUCTION_BREEZSERVER_URL.to_string(), None)
                .map_err(|e| e.to_string())?,
        );
        Ok(WasmPlayground {
            fiat_api: breez_server,
        })
    }

    #[wasm_bindgen(js_name = "fetchFiatRates")]
    pub async fn fetch_fiat_rates(&self) -> Result<Vec<JsValue>, JsValue> {
        let rates = self
            .fiat_api
            .fetch_fiat_rates()
            .await
            .map_err(|e| e.to_string())?;
        Ok(rates.into_iter().map(|r| r.into()).collect())
    }

    #[wasm_bindgen(js_name = "listFiatCurrencies")]
    pub async fn list_fiat_currencies(&self) -> Result<Vec<JsValue>, JsValue> {
        let currencies = self
            .fiat_api
            .list_fiat_currencies()
            .await
            .map_err(|e| e.to_string())?;
        Ok(currencies.into_iter().map(|r| r.into()).collect())
    }

    #[wasm_bindgen(js_name = "parse")]
    pub async fn parse(&self, input: String) -> Result<InputType, JsValue> {
        let res = parse(&input, None).await.map_err(|e| e.to_string())?;
        Ok(res)
    }
}
