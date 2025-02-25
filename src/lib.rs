mod errors;
pub mod models;

use crate::models::{FiatCurrency, InputType, Rate, LNInvoice, WasmResult};
use sdk_common::input_parser::parse;
use sdk_common::invoice::parse_invoice;
use sdk_common::prelude::{BreezServer, FiatAPI, PRODUCTION_BREEZSERVER_URL};
use std::sync::Arc;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WasmPlayground {
    fiat_api: Arc<dyn FiatAPI>,
}

#[wasm_bindgen]
impl WasmPlayground {
    pub fn new() -> WasmResult<WasmPlayground> {
        let breez_server = Arc::new(BreezServer::new(
            PRODUCTION_BREEZSERVER_URL.to_string(),
            None,
        )?);
        Ok(WasmPlayground {
            fiat_api: breez_server,
        })
    }

    #[wasm_bindgen(js_name = "fetchFiatRates")]
    pub async fn fetch_fiat_rates(&self) -> WasmResult<Vec<Rate>> {
        let rates = self.fiat_api.fetch_fiat_rates().await?;
        Ok(rates.into_iter().map(|r| r.into()).collect())
    }

    #[wasm_bindgen(js_name = "listFiatCurrencies")]
    pub async fn list_fiat_currencies(&self) -> WasmResult<Vec<FiatCurrency>> {
        let currencies = self.fiat_api.list_fiat_currencies().await?;
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
}
