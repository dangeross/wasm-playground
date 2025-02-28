use std::collections::HashMap;

use anyhow::Result;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::sdk::model::{EventListener, SdkEvent};

pub(crate) struct EventManager {
    listeners: RwLock<HashMap<String, Box<dyn EventListener>>>,
    notifier: broadcast::Sender<SdkEvent>,
}

impl EventManager {
    pub fn new() -> Self {
        let (notifier, _) = broadcast::channel::<SdkEvent>(100);

        Self {
            listeners: Default::default(),
            notifier,
        }
    }

    pub async fn add(&self, listener: Box<dyn EventListener>) -> Result<String> {
        let id = Uuid::new_v4().to_string();
        (*self.listeners.write().await).insert(id.clone(), listener);
        Ok(id)
    }

    #[allow(dead_code)]
    pub async fn remove(&self, id: String) {
        (*self.listeners.write().await).remove(&id);
    }

    pub async fn notify(&self, e: SdkEvent) {
        let _ = self.notifier.send(e.clone());

        for listener in (*self.listeners.read().await).values() {
            listener.on_event(e.clone());
        }
    }

    #[allow(dead_code)]
    pub(crate) fn subscribe(&self) -> broadcast::Receiver<SdkEvent> {
        self.notifier.subscribe()
    }
}
