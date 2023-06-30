mod cadence;
mod events;
mod listeners;
mod notifiers;

use std::env;

use crate::listeners::{flow_listener::FlowNetwork, Requestable};

use byc_helpers::mongo::{
    self,
    models::{common::ModelCollection, Contract},
};
use futures::TryStreamExt;
use listeners::flow_listener::FlowListener;
use log::info;

#[tokio::main]
async fn main() {
    byc_helpers::logger::init_logger();
    // let args: Vec<String> = env::args().collect();
    let m_client = mongo::client::create().await;
    // if args.len() > 1 && args[1] == "feed" {
    // feed db

    let mut db_contract: Vec<String> = vec![];
    let contracts_col = Contract::get_collection(&m_client);
    let cursor = contracts_col.find(None, None).await.unwrap();
    let c_vec: Vec<Contract> = cursor.try_collect().await.unwrap();

    for c in c_vec.into_iter() {
        let mut s2 = Some("".to_string());
        println!("{}", c.id.clone());
        db_contract.push(c.id.clone());
        while s2.is_some() {
            s2 = gql::find_all_transactions(c.clone(), c.id.clone(), s2, &m_client).await;
            println!("{:?}", s2);
        }
    }
    // return;
    // let mut s = Some("".to_string());
    // while s.is_some() {
    //     s = gql::find_created_events(s, &m_client, &mut db_contract).await;
    //     println!("{:?}", s)
    // }
    // return;
    // }
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
