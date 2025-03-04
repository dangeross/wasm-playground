mod event;
pub mod model;

use std::sync::Arc;

use anyhow::Result;
use log::debug;
use sdk_common::prelude::{BreezServer, FiatAPI, FiatCurrency, Rate, PRODUCTION_BREEZSERVER_URL};

use crate::sdk::event::EventManager;
use crate::sdk::model::{EventListener, Payment, SdkEvent, Signer, SignerError};

struct MockSigner {}
impl Signer for MockSigner {
    fn xpub(&self) -> std::result::Result<Vec<u8>, SignerError> {
        Ok(Vec::new())
    }

    fn derive_xpub(&self, _derivation_path: String) -> std::result::Result<Vec<u8>, SignerError> {
        Ok(Vec::new())
    }

    fn sign_ecdsa(
        &self,
        _msg: Vec<u8>,
        _derivation_path: String,
    ) -> std::result::Result<Vec<u8>, SignerError> {
        Ok(Vec::new())
    }

    fn sign_ecdsa_recoverable(&self, _msg: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
        Ok(Vec::new())
    }

    fn slip77_master_blinding_key(&self) -> std::result::Result<Vec<u8>, SignerError> {
        Ok(Vec::new())
    }

    fn hmac_sha256(
        &self,
        _msg: Vec<u8>,
        _derivation_path: String,
    ) -> std::result::Result<Vec<u8>, SignerError> {
        Ok(Vec::new())
    }

    fn ecies_encrypt(&self, _msg: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
        Ok(Vec::new())
    }

    fn ecies_decrypt(&self, _msg: Vec<u8>) -> std::result::Result<Vec<u8>, SignerError> {
        Ok(Vec::new())
    }
}

pub struct Sdk {
    fiat_api: Arc<dyn FiatAPI>,
    event_manager: Arc<EventManager>,
    signer: Box<dyn Signer>,
}

impl Sdk {
    pub fn new() -> Result<Sdk> {
        Sdk::new_with_signer(Box::new(MockSigner {}))
    }

    pub fn new_with_signer(signer: Box<dyn Signer>) -> Result<Sdk> {
        let event_manager = Arc::new(EventManager::new());
        let breez_server = Arc::new(BreezServer::new(
            PRODUCTION_BREEZSERVER_URL.to_string(),
            None,
        )?);
        Ok(Sdk {
            event_manager,
            fiat_api: breez_server,
            signer,
        })
    }

    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>> {
        let key = self.signer.slip77_master_blinding_key()?;
        debug!("Key: {:?}", key);
        self.event_manager
            .notify(SdkEvent::PaymentSucceeded {
                details: Payment {
                    id: "mock payment id".to_string(),
                },
            })
            .await;
        self.fiat_api.fetch_fiat_rates().await
    }

    pub async fn list_fiat_currencies(&self) -> Result<Vec<FiatCurrency>> {
        self.fiat_api.list_fiat_currencies().await
    }

    pub async fn add_event_listener(&self, listener: Box<dyn EventListener>) -> Result<String> {
        self.event_manager.add(listener).await
    }
}
