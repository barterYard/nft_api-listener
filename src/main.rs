mod cadence;
mod events;
mod listeners;
mod notifiers;

use std::time::Duration;

use crate::listeners::{flow_listener::FlowNetwork, Requestable};

use flow_helpers::mongo::{
    self,
    models::{common::ModelCollection, mongo_doc, Contract},
};
use futures::TryStreamExt;
use listeners::flow_listener::FlowListener;
use log::info;

#[tokio::main]
async fn main() {
    flow_helpers::logger::init_logger();
    loop {
        let m_client = mongo::client::create().await;
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(4))
            .build()
            .unwrap();

        let contracts_col = Contract::get_collection(&m_client);
        let cursor = contracts_col.find(mongo_doc! {}, None).await.unwrap();

        let c_vec: Vec<Contract> = cursor.try_collect().await.unwrap();

        let mut db_contract: Vec<String> = c_vec.clone().into_iter().map(|x| x.id).collect();
        // find all created contract
        let mut s = Some("".to_string());
        while s.is_some() {
            s = gql::find_created_events(s, &m_client, &mut db_contract, &client).await;
        }

        let cursor = contracts_col.find(mongo_doc! {}, None).await.unwrap();
        let c_vec: Vec<Contract> = cursor.try_collect().await.unwrap();

        // db feeder that allow to feed the database with historical data (this is not real time)
        let mut contract_done = 0;

        for c in c_vec.clone().into_iter() {
            let mut s2 = Some(c.last_cursor.clone().unwrap_or_default());
            let mut total_nft = 0;
            info!("start {} ", c.identifier);

            while s2.is_some() {
                let x;
                c.update(
                    &m_client,
                    mongo_doc! {
                        "$set": {
                            "lastCursor": Some(s2.clone()),
                        }
                    },
                )
                .await;
                (s2, x) = gql::find_all_transactions(&c, s2, &m_client, &client).await;
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
        }
    }

    // blockchain listener
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
