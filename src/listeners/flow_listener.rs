use super::event_listener::EventListener;
use crate::listeners::Messageable;
use crate::notifiers::Notifier;
use flow_helpers::flow_sdk;
use flow_sdk::{entities::Event, prelude::TonicHyperFlowClient};

use log::{error, info};
use std::{env, error::Error};
use tokio::sync::mpsc::channel;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlowNetwork {
    Testnet,
    Mainnet,
}

impl FlowNetwork {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Mainnet => "mainnet",
            Self::Testnet => "testnet",
        }
    }
    fn from_string(network: String) -> FlowNetwork {
        match network.as_str() {
            "mainnet" => FlowNetwork::Mainnet,
            _ => FlowNetwork::Testnet,
        }
    }

    pub fn get() -> FlowNetwork {
        FlowNetwork::from_string(env::var("FLOW_ENV").unwrap_or_else(|_| "testnet".to_string()))
    }

    pub async fn get_flow_client(&self) -> TonicHyperFlowClient {
        match self {
            Self::Mainnet => TonicHyperFlowClient::mainnet().await.unwrap(),
            Self::Testnet => TonicHyperFlowClient::testnet().await.unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct FlowListener<'a> {
    event_listener: EventListener,
    pub notifier: Notifier<'a>,
}

pub trait Events {
    fn feed_events(_event_list: &mut Vec<Box<dyn Messageable>>, _events: (&str, Vec<Event>)) {
        panic!("must be implemented in client");
    }
}

// impl Events for FlowListener<'_> {
//     fn feed_events(event_list: &mut Vec<Box<dyn Messageable>>, events: (&str, Vec<Event>)) {
//         panic!("must be implemented in client");
//     }
// }

impl FlowListener<'_> {
    pub async fn create<'a>(
        notifier: Notifier<'a>,
        events: &mut Vec<&'static str>,
    ) -> Result<FlowListener<'a>, Box<dyn Error>> {
        // create flow mainnet client
        let network = FlowNetwork::get();
        let mut client = network.get_flow_client().await;
        client.ping().await?;
        // register events
        let mut w_events: Vec<&str> = notifier
            .webhooks
            .unwrap()
            .iter()
            .map(|w| w.event())
            .collect();

        w_events.append(events);
        w_events.dedup();
        let event_listener = EventListener {
            last_requested_block: 0,
            events: w_events,
            client,
        };
        Ok(FlowListener {
            event_listener,
            notifier,
        })
    }

    // Helper to register all events in one place

    pub async fn start(&mut self) {
        info!("start listener");

        let (mut sx, mut rx) = channel::<(&str, Vec<Event>)>(10);
        let runner = self.event_listener.set_listener_events(&mut sx);

        let event_listener = async {
            while let Some(events) = rx.recv().await {
                let event_list: &mut Vec<Box<dyn Messageable>> = &mut Vec::new();
                FlowListener::feed_events(event_list, events);

                for ev in event_list {
                    ev.send(&self.notifier).await;
                }
            }
            error!("broke pipe listener");
        };
        tokio::join!(runner, event_listener);
        error!("listener stopped");
    }
}
