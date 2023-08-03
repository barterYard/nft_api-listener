use flow_helpers::flow_sdk;
use flow_sdk::{entities::Event, prelude::TonicHyperFlowClient, protobuf::Seal};
use log::{error, info};
use std::error::Error;
use std::fmt;
use std::time;
use tokio::sync::mpsc;
use tokio::time::sleep;

use crate::events;
use crate::listeners::Requestable;

#[derive(Debug)]
struct EventListenerError {
    message: &'static str,
}

impl fmt::Display for EventListenerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message)
    }
}

impl Error for EventListenerError {
    fn description(&self) -> &str {
        self.message
    }
}

#[derive(Clone)]
pub struct EventListener {
    pub last_requested_block: u64,
    pub events: Vec<&'static str>,
    pub client: TonicHyperFlowClient,
}

impl EventListener {
    pub async fn set_listener_events(&mut self, sender: &mut mpsc::Sender<(&str, Vec<Event>)>) {
        info!("bc listener started for events {:?}", self.events);
        loop {
            self.events
                .append(&mut events::DepositEvent::get_event_types());
            self.events
                .append(&mut events::WithdrawEvent::get_event_types());
            self.events.sort();
            self.events.dedup();

            let mut latest_block = match self.client.latest_block_header(Seal::Sealed).await {
                Ok(block) => {
                    if self.last_requested_block > block.height {
                        sleep(time::Duration::from_millis(500)).await;
                        continue;
                    }
                    block.height
                }
                Err(err) => {
                    error!("{:?}", err);
                    sleep(time::Duration::from_millis(500)).await;
                    continue;
                }
            };
            if self.last_requested_block == 0 {
                self.last_requested_block = latest_block - 10;
            }
            if latest_block - self.last_requested_block > 250 {
                latest_block = self.last_requested_block + 200
            }
            for event_type in &self.events {
                let event_list: Vec<Event> = match self
                    .client
                    .events_for_height_range(event_type, self.last_requested_block, latest_block)
                    .await
                {
                    Ok(m) => m.results.into_iter().flat_map(|x| x.events).collect(),
                    Err(err) => {
                        error!("{:?}", err);
                        drop(err);
                        continue;
                    }
                };
                if event_list.is_empty() {
                    info!("{:?}", event_type);
                    continue;
                }

                if sender.is_closed() {
                    error!("sender closed");
                    continue;
                }
                match sender.send((event_type, event_list)).await {
                    Err(e) => {
                        error!("{:?}", e);
                        continue;
                    }
                    _ => continue,
                };
            }
            self.last_requested_block = latest_block + 1;
            sleep(time::Duration::from_millis(500)).await;
        }
        error!("stopped listener loop");
    }
}
