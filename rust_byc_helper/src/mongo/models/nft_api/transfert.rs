use crate::mongo::models::common::ModelCollection;
use bson::oid::ObjectId;
use mongodb::{error::Error, results::InsertOneResult, Client};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Transfert {
    pub _id: ObjectId,
    pub date: String,
    pub from: String,
    pub to: String,
    pub nft: ObjectId,
}

impl Transfert {
    pub fn new(date: String, from: String, to: String, nft: ObjectId) -> Self {
        Transfert {
            _id: ObjectId::new(),
            date,
            from,
            to,
            nft,
        }
    }
    pub async fn create(
        date: String,
        from: String,
        to: String,
        nft: ObjectId,
        client: &Client,
    ) -> Result<InsertOneResult, Error> {
        let transfer = Transfert {
            _id: ObjectId::new(),
            date,
            from,
            to,
            nft,
        };
        Transfert::get_collection(client)
            .insert_one(transfer, None)
            .await
    }

    pub async fn save(&self, client: &Client) -> Result<InsertOneResult, Error> {
        Transfert::get_collection(client)
            .insert_one(self, None)
            .await
    }
}
