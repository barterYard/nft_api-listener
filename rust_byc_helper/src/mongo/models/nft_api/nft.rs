use crate::mongo::models::{common::ModelCollection, mongo_doc};
use bson::oid::ObjectId;
use mongodb::{error::Error, results::UpdateResult, Client};
use proc::ModelCollection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone, ModelCollection)]
pub struct Nft {
    pub _id: ObjectId,
    pub id: String,
    pub description: Option<String>,
    pub name: Option<String>,
    pub burned: bool,
    pub contract: ObjectId,
}

impl Nft {
    pub async fn get_or_create(client: &Client, contract_id: ObjectId, nft_id: String) -> Nft {
        let nft_col = Nft::get_collection(client);
        match nft_col
            .find_one(
                mongo_doc! {"contract": contract_id, "id": nft_id.clone(), "burned": false},
                None,
            )
            .await
        {
            Ok(y) => match y {
                Some(nft) => return nft,
                _ => {
                    let new_nft = Nft {
                        contract: contract_id,
                        id: nft_id,
                        _id: bson::oid::ObjectId::new(),
                        ..Default::default()
                    };
                    let _ = nft_col.insert_one(&new_nft, None).await;
                    new_nft
                }
            },
            Err(_) => {
                let new_nft = Nft {
                    contract: contract_id,
                    id: nft_id,
                    _id: bson::oid::ObjectId::new(),
                    ..Default::default()
                };
                let _ = nft_col.insert_one(&new_nft, None).await;
                new_nft
            }
        }
    }
    pub async fn burn(&self, client: &Client) -> Result<UpdateResult, Error> {
        Nft::get_collection(client)
            .update_one(
                mongo_doc! {
                    "_id": self._id
                },
                mongo_doc! {"$set": mongo_doc! {
                    "burned": true,
                }},
                None,
            )
            .await
    }
    pub async fn mint(&self, client: &Client) -> Result<UpdateResult, Error> {
        Nft::get_collection(client)
            .update_one(
                mongo_doc! {
                    "_id": self._id
                },
                mongo_doc! {"$set": mongo_doc! {
                    "burned": false,
                }},
                None,
            )
            .await
    }
}
