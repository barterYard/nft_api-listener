use crate::listeners::{Cadencable, Messageable, Requestable};
use crate::notifiers::{BaseClient, Notifier};
use async_trait::async_trait;

use flow_helpers::flow_sdk;
use flow_helpers::mongo::models::common::ModelCollection;
use flow_helpers::mongo::models::Contract;
use flow_helpers::mongo::mongodb::bson;
use flow_sdk::prelude::cadence_json::{CompositeOwned, ValueOwned};
use serde::{Deserialize, Serialize};

use serde_json::json;

//pub event AccountContractAdded(address: Address,codeHash: [UInt8],contract: String)
type ContractAddedFields = (String, Vec<ValueOwned>, String);
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContractAddedEvent {
    pub address: String,
    pub code_hash: Vec<ValueOwned>,
    pub contract: String,
}

trait ToField {
    fn get_listing_field(&self) -> ContractAddedFields;
}

impl ToField for CompositeOwned {
    fn get_listing_field(&self) -> ContractAddedFields {
        let address = if let ValueOwned::Address(x) = self.find_field("address").unwrap() {
            x.to_string()
        } else {
            "".to_string()
        };
        let code_hash = if let ValueOwned::Array(k) = self.find_field("codeHash").unwrap() {
            k.to_owned()
        } else {
            vec![]
        };

        let contract = match self.find_field("contract") {
            Some(ValueOwned::String(x)) => x.to_owned(),
            _ => "".to_owned(),
        };

        (address, code_hash, contract)
    }
}

impl Cadencable for ContractAddedEvent {
    fn from_cadence(obj: &CompositeOwned) -> ContractAddedEvent {
        let (address, code_hash, contract) = obj.get_listing_field();

        ContractAddedEvent {
            address,
            code_hash,
            contract,
        }
    }
}

impl Requestable for ContractAddedEvent {
    fn get_event_types() -> Vec<String> {
        vec!["flow.AccountContractAdded".to_string()]
    }
}

#[async_trait(?Send)]
impl Messageable for ContractAddedEvent {
    async fn to_message(&self) -> Option<serde_json::Value> {
        Some(json!({}))
    }

    async fn send(&self, notifier: &Notifier) {
        if let Some(db) = notifier.database {
            let contract_collection = Contract::get_collection(db);
            let contract = Contract {
                _id: bson::oid::ObjectId::new(),
                address: "".to_string(),
                id: "".to_string(),
                contract_type: "".to_string(),
                ..Default::default()
            };
            let _ = contract_collection.insert_one(contract, None).await;
        }
        if let Some(webhooks) = notifier.webhooks {
            if let Some(webhook) = webhooks
                .iter()
                .find(|x| Self::get_event_types().contains(&x.event()))
            {
                webhook.send_message(self).await;
            }
        }
    }
}
