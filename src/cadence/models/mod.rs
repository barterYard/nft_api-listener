mod storefront_listing;

use flow_helpers::flow_sdk;
use flow_sdk::prelude::cadence_json::{CompositeOwned, ValueOwned};

pub use storefront_listing::StorefrontListing;

pub trait CompositeHelper {
    fn unwrap_optional(&self, value: &'static str) -> Option<ValueOwned>;
}

impl CompositeHelper for CompositeOwned {
    fn unwrap_optional(&self, value: &'static str) -> Option<ValueOwned> {
        match self.find_field(value) {
            Some(ValueOwned::Optional(r)) => r.to_owned().map(|x| *x),
            None => None,
            x => Some(x.unwrap().to_owned()),
        }
    }
}
impl CompositeHelper for ValueOwned {
    fn unwrap_optional(&self, _value: &'static str) -> Option<ValueOwned> {
        match self {
            ValueOwned::Optional(r) => r.clone().map(|x| *x),
            x => Some(x.to_owned()),
        }
    }
}
