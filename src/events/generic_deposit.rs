use crate::listeners::{Cadencable, Messageable, Requestable};
use crate::notifiers::webhook::client;
use crate::notifiers::{BaseClient, Notifier};
use async_trait::async_trait;

use flow_helpers::flow_sdk;
use flow_helpers::mongo;
use flow_helpers::mongo::models::common::ModelCollection;
use flow_helpers::mongo::models::{mongo_doc, Contract};
use flow_helpers::mongo::mongodb::options::FindOptions;
use flow_sdk::prelude::cadence_json::{CompositeOwned, ValueOwned};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};

use serde_json::json;

//pub event Deposit(id: UInt64, to: Address?)
type DepositEventFields = (u64, Option<String>);
#[derive(Debug, Serialize, Deserialize)]
pub struct DepositEvent {
    pub id: u64,
    pub to: Option<String>,
}

trait ToField {
    fn get_listing_field(&self) -> DepositEventFields;
}

impl ToField for CompositeOwned {
    fn get_listing_field(&self) -> DepositEventFields {
        let to = if let ValueOwned::Address(x) = self.find_field("to").unwrap() {
            Some(x.to_string())
        } else {
            None
        };
        let id = if let ValueOwned::UInt64(k) = self.find_field("id").unwrap() {
            k.to_owned()
        } else {
            0
        };
        (id, to)
    }
}

impl Cadencable for DepositEvent {
    fn from_cadence(obj: &CompositeOwned) -> DepositEvent {
        let (id, to) = obj.get_listing_field();

        DepositEvent { id, to }
    }
}

impl Requestable for DepositEvent {
    fn get_event_types() -> Vec<String> {
        // read db

        vec![".Deposit".to_string()]
    }
}

#[async_trait(?Send)]
impl Messageable for DepositEvent {
    async fn to_message(&self) -> Option<serde_json::Value> {
        Some(json!({}))
    }

    async fn send(&self, notifier: &Notifier) {
        if let Some(db) = notifier.database {}
    }
}
