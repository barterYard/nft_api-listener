use crate::listeners::{Cadencable, Messageable, Requestable};
use crate::notifiers::{BaseClient, Notifier};
use async_trait::async_trait;

use flow_helpers::flow_sdk;
use flow_helpers::mongo::models::common::ModelCollection;
use flow_helpers::mongo::models::Contract;
use flow_sdk::prelude::cadence_json::{CompositeOwned, ValueOwned};
use serde::{Deserialize, Serialize};

use serde_json::json;

// pub event Withdraw(id: UInt64, from: Address?)
type WithdrawEventFields = (u64, Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct WithdrawEvent {
    pub id: u64,
    pub from: Option<String>,
}

trait ToField {
    fn get_listing_field(&self) -> WithdrawEventFields;
}

impl ToField for CompositeOwned {
    fn get_listing_field(&self) -> WithdrawEventFields {
        let from = if let ValueOwned::Address(x) = self.find_field("from").unwrap() {
            Some(x.to_string())
        } else {
            None
        };
        let id = if let ValueOwned::UInt64(k) = self.find_field("id").unwrap() {
            k.to_owned()
        } else {
            0
        };
        (id, from)
    }
}

impl Cadencable for WithdrawEvent {
    fn from_cadence(obj: &CompositeOwned) -> WithdrawEvent {
        let (id, from) = obj.get_listing_field();
        WithdrawEvent { id, from }
    }
}

impl Requestable for WithdrawEvent {
    fn get_event_types() -> Vec<String> {
        // read db
        vec![".Withdraw".to_string()]
    }
}

#[async_trait(?Send)]
impl Messageable for WithdrawEvent {
    async fn to_message(&self) -> Option<serde_json::Value> {
        Some(json!({}))
    }

    async fn send(&self, notifier: &Notifier) {
        if let Some(db) = notifier.database {}
    }
}
