mod contract_added;
mod generic_deposit;
mod generic_withdraw;

use crate::listeners::{
    flow_listener::{Events, FlowListener},
    Messageable, Requestable,
};
use contract_added::ContractAddedEvent;
use flow_sdk::entities::Event;
pub use generic_deposit::DepositEvent;
pub use generic_withdraw::WithdrawEvent;

impl Events for FlowListener<'_> {
    fn feed_events(event_list: &mut Vec<Box<dyn Messageable>>, events: (&str, Vec<Event>)) {
        ContractAddedEvent::feed_events(event_list, &events);
        WithdrawEvent::feed_events(event_list, &events);
        DepositEvent::feed_events(event_list, &events);
        // ListingEvent::feed_events(event_list, &events);
        // TopShotEvent::feed_events(event_list, &events);
        // MintingEvent::feed_events(event_list, &events);
        // BarterCreatedEvent::feed_events(event_list, &events);
        // BarterDestroyedEvent::feed_events(event_list, &events);
        // BarterExecutedEvent::feed_events(event_list, &events);
    }
}
