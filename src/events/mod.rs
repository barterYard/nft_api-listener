mod all_day_minting;
mod gaia_listing;
mod topshot_event;

mod barter_created;
mod barter_destroyed;
mod barter_executed;

pub use barter_created::BarterCreatedEvent;
pub use barter_destroyed::BarterDestroyedEvent;
pub use barter_executed::BarterExecutedEvent;

pub use all_day_minting::MintingEvent;
use flow_sdk::entities::Event;
pub use gaia_listing::ListingEvent;
pub use topshot_event::TopShotEvent;

use crate::listeners::{
    flow_listener::{Events, FlowListener},
    Messageable, Requestable,
};

impl Events for FlowListener<'_> {
    fn feed_events(event_list: &mut Vec<Box<dyn Messageable>>, events: (&str, Vec<Event>)) {
        ListingEvent::feed_events(event_list, &events);
        TopShotEvent::feed_events(event_list, &events);
        MintingEvent::feed_events(event_list, &events);
        BarterCreatedEvent::feed_events(event_list, &events);
        BarterDestroyedEvent::feed_events(event_list, &events);
        BarterExecutedEvent::feed_events(event_list, &events);
    }
}
