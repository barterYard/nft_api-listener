mod cadence;
mod events;
mod listeners;

use std::env;

use crate::{
    events::{ListingEvent, TopShotEvent},
    listeners::{flow_listener::FlowNetwork, Requestable},
};

use listeners::flow_listener::FlowListener;
use log::info;

#[tokio::main]
async fn main() {
    byc_helpers::logger::init_logger();

    // create notifiers events
    let mut webhooks = vec![];
    if env::var("BARTER_LISTENER").is_ok() {
        webhooks = vec![
            WebhookClient {
                mainnet_event: "A.b18b1dc5069a41c7.BYC.BarterCreated",
                testnet_event: "A.6cc20ec6609bad7f.BYC.BarterCreated",
                testnet_endpoint: "https://staging-service.barteryard.club/barters/created",
                mainnet_endpoint: "https://service.barteryard.club/barters/created",
            },
            WebhookClient {
                mainnet_event: "A.b18b1dc5069a41c7.BYC.BarterDestroyed",
                testnet_event: "A.6cc20ec6609bad7f.BYC.BarterDestroyed",
                testnet_endpoint: "https://staging-service.barteryard.club/barters/destroyed",
                mainnet_endpoint: "https://service.barteryard.club/barters/destroyed",
            },
            WebhookClient {
                mainnet_event: "A.b18b1dc5069a41c7.BYC.BarterExecuted",
                testnet_event: "A.6cc20ec6609bad7f.BYC.BarterExecuted",
                testnet_endpoint: "https://staging-service.barteryard.club/barters/executed",
                mainnet_endpoint: "https://service.barteryard.club/barters/executed",
            },
        ];
    }

    let events: &mut Vec<&str> = &mut vec![];
    let network = FlowNetwork::get();
    info!("server started on flow {:?}", network);
    if network == FlowNetwork::Mainnet && env::var("BARTER_LISTENER").is_err() {
        events.append(&mut ListingEvent::get_event_types());
        events.append(&mut TopShotEvent::get_event_types());
    }

    // create and run server
    FlowListener::create(
        notifiers::Notifier {
            discord: Some(&BycDiscordClient {}),
            webhooks: Some(&webhooks),
        },
        events,
    )
    .await
    .unwrap()
    .start()
    .await;
}
