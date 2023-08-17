use std::time::Duration;

use async_trait::async_trait;
use log::error;

use crate::{
    listeners::{flow_listener::FlowNetwork, Messageable},
    notifiers::BaseClient,
};
#[derive(Debug, Clone, Copy)]
pub struct WebhookClient {
    pub mainnet_endpoint: &'static str,
    pub mainnet_event: &'static str,
    pub testnet_event: &'static str,
    pub testnet_endpoint: &'static str,
}

impl WebhookClient {
    pub fn endpoint(&self) -> &'static str {
        match FlowNetwork::get() {
            FlowNetwork::Testnet => self.testnet_endpoint,
            FlowNetwork::Mainnet => self.mainnet_endpoint,
        }
    }
    pub fn event(&self) -> String {
        match FlowNetwork::get() {
            FlowNetwork::Testnet => self.testnet_event.to_string(),
            FlowNetwork::Mainnet => self.mainnet_event.to_string(),
        }
    }
}

#[async_trait(?Send)]
impl<T: Messageable> BaseClient<T> for WebhookClient {
    async fn send_message(&self, messageable: &T) {
        let msg = messageable.to_message().await;
        let url = self.endpoint();
        if msg.is_none() {
            return;
        }
        if let Some(w_msg) = msg.unwrap().as_object() {
            let client = reqwest::Client::builder()
                .timeout(Duration::from_secs(15))
                .build()
                .unwrap();
            let mut retry = 0;
            while retry < 3 {
                match client.post(url).json(w_msg).send().await {
                    Ok(_) => break,
                    Err(err) => {
                        retry += 1;
                        error!("{:?}", err);
                        drop(err);
                    }
                };
            }
        }
    }
}
