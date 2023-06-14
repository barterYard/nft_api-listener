use std::{any::Any, fmt::Display};

use async_trait::async_trait;
use flow_sdk::{entities::Event, prelude::cadence_json::CompositeOwned};

use crate::{
    events::{ListingEvent, MintingEvent, TopShotEvent},
    notifiers::Notifier,
};

pub trait Cadencable {
    fn from_cadence(obj: &CompositeOwned) -> Self;
}

#[async_trait(?Send)]
pub trait Requestable
where
    Self: Sized,
{
    fn get_event_types() -> Vec<&'static str>;
    fn get_events_from(events: &(&str, Vec<Event>)) -> Vec<Self>
    where
        Self: Cadencable,
    {
        if !Self::get_event_types().contains(&events.0) {
            return vec![];
        }
        let mut result: Vec<Self> = vec![];
        for event in events.to_owned().1 {
            let parsed = event.parse_payload();

            if parsed.as_ref().ok() != None {
                let parsed_event = parsed.ok().unwrap();
                result.push(Self::from_cadence(&parsed_event));
            } else {
                println!("{:?}", parsed.err());
            }
        }
        result
    }
    fn feed_events(event_list: &mut Vec<Box<dyn Messageable>>, events: &(&str, Vec<Event>))
    where
        Self: Requestable + Cadencable + Messageable,
    {
        if !Self::get_event_types().contains(&events.0) {
            return;
        }
        Self::get_events_from(events)
            .into_iter()
            .for_each(|ev| event_list.push(Box::new(ev)));
    }
}

#[async_trait(?Send)]
pub trait Messageable: MessageableToAny {
    async fn to_message(&self) -> Option<serde_json::Value>;
    async fn send(&self, notifier: &Notifier);
}
pub trait MessageableToAny: 'static {
    fn as_any(&self) -> &dyn Any;
}

impl Display for dyn Messageable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(e) = self.as_any().downcast_ref::<ListingEvent>() {
            return f.write_str(format!("ListingEvent {}", e.channel).as_str());
        }
        if let Some(e) = self.as_any().downcast_ref::<MintingEvent>() {
            return f.write_str(format!("MintingEvent {}", e.channel).as_str());
        }
        if let Some(e) = self.as_any().downcast_ref::<TopShotEvent>() {
            return f.write_str(format!("TopShotEvent {}", e.channel).as_str());
        }
        f.write_str("Messageable")
    }
}
impl<T: 'static> MessageableToAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
