use std::collections::HashMap;

use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::{oid::ObjectId, Document};
use log::info;
use mongodb::{error::Error, results::UpdateResult, Client, ClientSession};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Owner {
    pub _id: ObjectId,
    pub address: String,
    pub nfts: HashMap<String, Vec<ObjectId>>,
}

impl Owner {
    pub fn new(address: String, nfts: Option<HashMap<String, Vec<ObjectId>>>) -> Self {
        Owner {
            _id: ObjectId::new(),
            address,
            nfts: nfts.unwrap_or_default(),
        }
    }

    pub async fn get_or_create(
        client: &Client,
        address: String,
        session: Option<&mut ClientSession>,
    ) -> Self {
        let owner_col = Owner::get_collection(client);
        let address = match address.as_str() {
            "null" => "0x0".to_string(),
            _ => address,
        };
        match owner_col
            .find_one(mongo_doc! {"address": &address}, None)
            .await
        {
            Ok(Some(owner)) => return owner,
            _ => {
                let new_owner = Owner {
                    address,
                    _id: bson::oid::ObjectId::new(),
                    ..Default::default()
                };
                let _ = match session {
                    Some(s) => owner_col.insert_one_with_session(&new_owner, None, s).await,
                    _ => owner_col.insert_one(&new_owner, None).await,
                };

                new_owner
            }
        }
    }

    pub async fn update(
        &self,
        operation: Document,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        let o_col = Owner::get_collection(client);
        let q = mongo_doc! {"_id": self._id};
        match session {
            Some(s) => o_col.update_one_with_session(q, operation, None, s).await,
            _ => o_col.update_one(q, operation, None).await,
        }
    }

    pub async fn add_owned_nft(
        &mut self,
        nft: ObjectId,
        contract_id: String,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        let field_name = "nfts.".to_owned() + &contract_id.replace(".", "_");

        let op = mongo_doc! {
            "$addToSet": {field_name: nft}
        };
        self.update(op, client, session).await
    }

    pub async fn remove_owned_nft(
        &mut self,
        contract_id: String,
        nft: ObjectId,
        client: &Client,
        session: Option<&mut ClientSession>,
    ) -> Result<UpdateResult, Error> {
        let field_name = "nfts.".to_owned() + &contract_id.replace(".", "_");
        let op = mongo_doc! {
            "$pull": {field_name: nft}
        };
        self.update(op, client, session).await
    }
}
