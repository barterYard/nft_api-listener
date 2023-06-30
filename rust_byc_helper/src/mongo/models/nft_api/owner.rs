use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::oid::ObjectId;
use mongodb::{
    error::Error,
    results::{InsertOneResult, UpdateResult},
    Client,
};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Owner {
    pub _id: ObjectId,
    pub address: String,
    pub nfts: Option<Vec<ObjectId>>,
}

impl Owner {
    pub fn new(address: String, nfts: Option<Vec<ObjectId>>) -> Self {
        Owner {
            _id: ObjectId::new(),
            address,
            nfts,
        }
    }
    pub async fn update(&self, client: &Client) -> Result<UpdateResult, Error> {
        Owner::get_collection(client)
            .update_one(
                mongo_doc! {
                    "_id": self._id
                },
                mongo_doc! {"$set": mongo_doc! {
                    "nfts": self.nfts.clone(),
                }},
                None,
            )
            .await
    }
    pub async fn get_or_create(client: &Client, address: String) -> Self {
        let owner_col = Owner::get_collection(client);
        let mut address = address.clone();
        if address == "null" {
            address = "0x0".to_string();
        }
        match owner_col
            .find_one(mongo_doc! {"address": &address}, None)
            .await
        {
            Ok(y) => match y {
                Some(owner) => return owner,
                _ => {
                    let new_owner = Owner {
                        address,
                        _id: bson::oid::ObjectId::new(),
                        ..Default::default()
                    };
                    let _ = owner_col.insert_one(&new_owner, None).await;
                    new_owner
                }
            },
            Err(_) => {
                let new_owner = Owner {
                    address,
                    _id: bson::oid::ObjectId::new(),
                    ..Default::default()
                };
                let _ = owner_col.insert_one(&new_owner, None).await;
                new_owner
            }
        }
    }

    pub async fn add_owned_nft(
        &mut self,
        nft: ObjectId,
        client: &Client,
    ) -> Result<UpdateResult, Error> {
        if self.nfts.is_none() {
            self.nfts = Some(vec![nft])
        } else {
            let mut new_nfts = self.nfts.clone().unwrap();
            new_nfts.push(nft);
            self.nfts = Some(new_nfts);
        }
        self.update(client).await
    }

    pub async fn remove_owned_nft(
        &mut self,
        nft: ObjectId,
        client: &Client,
    ) -> Result<UpdateResult, Error> {
        if self.nfts.is_none() {
            self.nfts = Some(vec![]);
        }
        let new_nfts = self.nfts.clone().unwrap();
        self.nfts = Some(
            new_nfts
                .into_iter()
                .filter(|x| x.to_string() != nft.to_string())
                .collect(),
        );
        self.update(client).await
    }
}
