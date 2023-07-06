mod cadence;
mod events;
mod listeners;
mod notifiers;

use std::{env, fs::File, time::Duration};

use crate::listeners::{flow_listener::FlowNetwork, Requestable};

use byc_helpers::mongo::{
    self,
    models::{common::ModelCollection, create_nft_api_db, mongo_doc, Contract, Transfer},
    mongodb::{options::IndexOptions, IndexModel},
};
use futures::TryStreamExt;
use listeners::flow_listener::FlowListener;
use log::info;

#[tokio::main]
async fn main() {
    byc_helpers::logger::init_logger();

    let m_client = mongo::client::create().await;
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(4))
        .build()
        .unwrap();

    // clear db
    // create_nft_api_db(&m_client).await;
    // return;

    let contracts_col = Contract::get_collection(&m_client);
    let cursor = contracts_col
        .find(mongo_doc! {"done": false}, None)
        .await
        .unwrap();
    let c_vec: Vec<Contract> = cursor.try_collect().await.unwrap();
    // let db_contract: Vec<String> = c_vec.clone().into_iter().map(|x| x.id).collect();

    // let mut s = Some("".to_string());

    // while s.is_some() {
    // s = gql::find_created_events(s, &m_client, &mut db_contract, &client).await;
    // }

    let mut contract_done = 0;

    for c in c_vec.clone().into_iter() {
        // if c.id == "A.a039bd7d55a96c0c.DriverzNFT" {
        let mut s2 = Some(c.last_cursor.clone().unwrap_or_default());
        let mut total_nft = 0;
        info!("start {} ", c.identifier);
        while s2.is_some() {
            let x;
            (s2, x) = gql::verify_transactions(c.clone(), s2, &m_client, &client).await;
            if s2.is_some() {
                c.clone()
                    .update(
                        &m_client,
                        mongo_doc! {
                            "$set": {
                                "lastCursor": Some(s2.clone()),
                            }
                        },
                    )
                    .await;
            }
            total_nft += x;
        }
        contract_done += 1;
        c.update(
            &m_client,
            mongo_doc! {
                "$set": {
                    "done": true,
                }
            },
        )
        .await;
        info!(
            "contract {} done {}/{} with {} transactions",
            c.identifier,
            contract_done,
            c_vec.len(),
            total_nft
        );
        // }
    }

    // let events: &mut Vec<&str> = &mut vec!["flow.AccountContractAdded"];

    // let network = FlowNetwork::get();
    // info!("server started on flow {:?}", network);

    // // create and run server
    // FlowListener::create(
    //     notifiers::Notifier {
    //         webhooks: Some(&vec![]),
    //         database: Some(&m_client),
    //     },
    //     events,
    // )
    // .await
    // .unwrap()
    // .start()
    // .await;
}
