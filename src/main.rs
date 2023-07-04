mod cadence;
mod events;
mod listeners;
mod notifiers;

use std::{env, fs::File, time::Duration};

use crate::listeners::{flow_listener::FlowNetwork, Requestable};

use byc_helpers::mongo::{
    self,
    models::{common::ModelCollection, create_nft_api_db, Contract},
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
    // if let Err(f) = File::open("already_run") {
    //     println!("{}", f);
    //     let _ = File::create("already_run");
    //     create_nft_api_db(&m_client).await;
    // } else {
    //     println!("crashed");
    //     // return;
    // }

    let contracts_col = Contract::get_collection(&m_client);
    let cursor = contracts_col.find(None, None).await.unwrap();
    let c_vec: Vec<Contract> = cursor.try_collect().await.unwrap();
    // let db_contract: Vec<String> = c_vec.clone().into_iter().map(|x| x.id).collect();

    // let mut s = Some("".to_string());

    // while s.is_some() {
    //     s = gql::find_created_events(s, &m_client, &mut db_contract, &client).await;
    // }

    let mut contract_done = 0;
    // c_vec.reverse();
    for c in c_vec.clone().into_iter() {
        // if c.id == "A.28abb9f291cadaf2.BarterYardClubWerewolf" {
        let mut s2 = Some("".to_string());

        info!("start {} ", c.identifier);
        while s2.is_some() {
            s2 = gql::find_all_transactions(c.clone(), c.id.clone(), s2, &m_client, &client).await;
        }
        contract_done += 1;
        info!(
            "contract {} done {}/{}",
            c.identifier,
            contract_done,
            c_vec.len()
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
