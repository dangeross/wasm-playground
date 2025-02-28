mod event;
pub mod model;

use anyhow::Result;
use sdk_common::prelude::{BreezServer, FiatAPI, FiatCurrency, Rate, PRODUCTION_BREEZSERVER_URL};
use std::sync::Arc;

use crate::sdk::event::EventManager;
use crate::sdk::model::{EventListener, Payment, SdkEvent};

pub struct Sdk {
    fiat_api: Arc<dyn FiatAPI>,
    event_manager: Arc<EventManager>,
}

impl Sdk {
    pub fn new() -> Result<Sdk> {
        let event_manager = Arc::new(EventManager::new());
        let breez_server = Arc::new(BreezServer::new(
            PRODUCTION_BREEZSERVER_URL.to_string(),
            None,
        )?);
        Ok(Sdk {
            event_manager,
            fiat_api: breez_server,
        })
    }

    pub async fn fetch_fiat_rates(&self) -> Result<Vec<Rate>> {
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
