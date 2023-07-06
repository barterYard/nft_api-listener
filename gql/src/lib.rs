use std::fmt;
use std::time::Duration;
use std::{collections::HashMap, error::Error};

use byc_helpers::mongo::{
    models::{
        common::ModelCollection, mongo_doc, Contract, DateTime, Deployment, GenNft, Owner, Transfer,
    },
    mongodb::{bson, Client},
};
use graphql_client::{GraphQLQuery, Response};

pub mod get_contract;
pub mod get_created_contract;
pub mod get_deposit;
pub mod get_transact;

use get_contract::getContract;
use get_created_contract::getCreatedContracts;
use get_deposit::getDepositEvent;
use get_transact::nftTransfer;
use log::{error, info};
use tokio::time::{self, sleep};

use crate::get_transact::nft_transfer::NftTransferNftTransfersEdges;

const FLOWGRAPH_URL: &str =
    "https://query.flowgraph.co/?token=5a477c43abe4ded25f1e8cc778a34911134e0590";

#[derive(Debug)]
struct GqlError {
    message: String,
}

impl fmt::Display for GqlError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.message.as_str())
    }
}

impl Error for GqlError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}
pub async fn find_contract(
    contract_id: String,
    db_client: &Client,
    db_contract: &mut Vec<String>,
    client: &reqwest::Client,
) {
    let variables = crate::get_contract::get_contract::Variables {
        id: contract_id.clone(),
    };
    let query = getContract::build_query(variables);

    let res = match client.post(FLOWGRAPH_URL).json(&query).send().await {
        Ok(x) => x,
        _ => return,
    };
    let response_body: Response<<getContract as GraphQLQuery>::ResponseData> =
        res.json().await.unwrap();
    let contract = match response_body.data {
        Some(x) => match x.contract {
            Some(c) => c,
            _ => return,
        },
        _ => return,
    };
    match contract.type_ {
        get_contract::get_contract::ContractType::NonFungibleToken => {
            db_contract.push(contract.id.clone());
            let mut deps: Vec<Deployment> = vec![];
            for dep in contract.deployments.unwrap().edges {
                deps.push(Deployment {
                    time: DateTime::parse_rfc3339_str(dep.node.unwrap().time).unwrap(),
                })
            }
            let db_ctr = Contract {
                _id: bson::oid::ObjectId::new(),
                address: contract.address.clone(),
                id: contract.id.clone(),
                locked: contract.locked,
                deleted: contract.deleted,
                identifier: contract.identifier.clone(),
                contract_type: format!("{:?}", contract.type_),
                deployments: deps,
                done: false,
            };

            let contract_col = Contract::get_collection(db_client);
            match contract_col
                .find_one(
                    mongo_doc! {
                        "id": contract.id
                    },
                    None,
                )
                .await
            {
                Ok(Some(_)) => {
                    info!("contract {} already exist!!", contract_id);
                }
                _ => {
                    let res = contract_col.insert_one(db_ctr, None).await;
                    if res.is_err() {
                        error!("{}", res.unwrap_err());
                    } else {
                        info!("contract {} added!!", contract_id);
                    }
                }
            };
        }

        _ => {}
    };
}

pub async fn find_created_events(
    after: Option<String>,
    db_client: &Client,
    db_contract: &mut Vec<String>,
    client: &reqwest::Client,
) -> Option<String> {
    let mut c = after.clone();
    if c.clone().unwrap_or("".to_string()) == "" {
        c = None;
    }

    let variables = get_created_contract::get_created_contracts::Variables { after: c.clone() };

    let query = getCreatedContracts::build_query(variables);

    let res = match client.post(FLOWGRAPH_URL).json(&query).send().await {
        Ok(x) => x,
        Err(e) => {
            error!("{:?}", e);
            return after;
        }
    };
    let response_body: Response<<getCreatedContracts as GraphQLQuery>::ResponseData> =
        match res.json().await {
            Ok(x) => x,
            Err(_) => return after,
        };

    let contract_events = response_body.data.unwrap().events.unwrap();

    for edge in contract_events.edges {
        let node = edge.clone().node.unwrap().clone();

        let address = node.fields[0].as_object().unwrap()["value"]
            .as_str()
            .clone()
            .unwrap();

        let name = node.fields[2].as_object().unwrap()["value"]
            .as_str()
            .clone()
            .unwrap();
        let contract_id = format!("A.{}.{}", address.replace("0x", ""), name);

        if !db_contract.contains(&contract_id) {
            find_contract(contract_id, db_client, db_contract, client).await;
        }
    }
    if contract_events.page_info.has_next_page {
        Some(contract_events.page_info.end_cursor)
    } else {
        None
    }
}

pub async fn find_event(
    contract_id: &String,
    ev: String,
    after: Option<String>,
    _db_client: &Client,
) -> Option<String> {
    let t_id = Some(contract_id.clone() + &ev.clone());

    let mut c = after;
    if c.clone().unwrap_or("".to_string()) == "" {
        c = None;
    }
    let variables = get_deposit::get_deposit_event::Variables {
        type_id: t_id,
        after: c.clone(),
    };
    let query = getDepositEvent::build_query(variables);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(4))
        .build()
        .unwrap();
    let res = match client.post(FLOWGRAPH_URL).json(&query).send().await {
        Ok(x) => x,
        _ => return c,
    };

    let response_body: Response<<getDepositEvent as GraphQLQuery>::ResponseData> =
        res.json().await.unwrap();
    let events = match response_body.data.unwrap().events {
        Some(x) => x,
        _ => return None,
    };
    for event in events.edges {
        info!("{:?}", event.node.unwrap().fields);
    }
    if events.page_info.has_next_page {
        Some(events.page_info.end_cursor)
    } else {
        None
    }
}

pub async fn verify_transactions(
    contract: Contract,
    after: Option<String>,
    db_client: &Client,
    client: &reqwest::Client,
) -> (Option<String>, u64) {
    let mut c = after.clone();
    if c.clone().unwrap_or("".to_string()) == "" {
        c = None;
    }
    let variables = get_transact::nft_transfer::Variables {
        after: c.clone(),
        contract_id: Some(contract.id.clone()),
    };
    let query = nftTransfer::build_query(variables);

    let res = match client.post(FLOWGRAPH_URL).json(&query).send().await {
        Ok(x) => x,
        Err(_) => {
            sleep(time::Duration::from_millis(500)).await;
            return (after, 0);
        }
    };
    let response_body: Response<<nftTransfer as GraphQLQuery>::ResponseData> =
        match res.json().await {
            Ok(x) => x,
            _ => return (after, 0),
        };

    let events = match response_body.data {
        Some(x) => x.nft_transfers,
        _ => return (after, 0),
    };
    for event in events.edges.clone() {
        let tra = event.node.unwrap();
        let from = tra.from.unwrap_or_default().address;
        let to = tra.to.unwrap_or_default().address;

        let (nft, created) =
            GenNft::get_or_create(db_client, &contract, tra.nft.nft_id.clone(), false, None).await;
        match Transfer::find(
            tra.transaction.time,
            from.clone(),
            to.clone(),
            nft._id,
            db_client,
        )
        .await
        {
            Some(x) => println!("Found"),
            None => {
                println!("Not Found")
            }
        };
    }
    if events.page_info.has_next_page {
        (Some(events.page_info.end_cursor), 0)
    } else {
        (None, 0)
    }
}

pub async fn find_all_transactions(
    contract: Contract,
    contract_id: String,
    after: Option<String>,
    db_client: &Client,
    client: &reqwest::Client,
) -> (Option<String>, usize) {
    let mut c = after.clone();
    if c.clone().unwrap_or("".to_string()) == "" {
        c = None;
    }
    let variables = get_transact::nft_transfer::Variables {
        after: c.clone(),
        contract_id: Some(contract_id.clone()),
    };
    let query = nftTransfer::build_query(variables);

    let res = match client.post(FLOWGRAPH_URL).json(&query).send().await {
        Ok(x) => x,
        Err(_) => {
            sleep(time::Duration::from_millis(500)).await;
            return (after, 0);
        }
    };
    let response_body: Response<<nftTransfer as GraphQLQuery>::ResponseData> =
        match res.json().await {
            Ok(x) => x,
            _ => return (after, 0),
        };

    let events = match response_body.data {
        Some(x) => x.nft_transfers,
        _ => return (after, 0),
    };
    let mut should_restart = false;
    let mut dup: Vec<String> = events
        .edges
        .clone()
        .into_iter()
        .filter_map(|x| {
            if x.node.is_some() {
                return Some(x.node.unwrap().nft.nft_id);
            } else {
                should_restart = true;
            }
            None
        })
        .collect();
    if should_restart {
        return (after, 0);
    }
    dup.sort();
    let mut map = HashMap::new();
    for e in dup.clone() {
        map.entry(e.clone()).or_insert(vec![]).push(e.clone());
    }
    dup.dedup();
    let r: Vec<String> = map
        .values()
        .clone()
        .into_iter()
        .filter_map(|x| {
            if x.len() > 1 {
                Some(x[0].clone())
            } else {
                None
            }
        })
        .collect();

    let fut_events: Vec<NftTransferNftTransfersEdges> = events
        .edges
        .clone()
        .into_iter()
        .filter(|x| !r.contains(&x.node.clone().unwrap().nft.nft_id))
        .collect();

    let mut count = 0;
    if dup.len() != events.edges.len() {
        let dup_event: Vec<NftTransferNftTransfersEdges> = events
            .edges
            .clone()
            .into_iter()
            .filter(|x| r.contains(&x.node.clone().unwrap().nft.nft_id))
            .collect();

        for event in dup_event.clone() {
            create_transfer(event, db_client, &contract).await;
        }
        count += dup_event.len();
    }

    let mut futs = vec![];
    for event in fut_events {
        futs.push(create_transfer(event, db_client, &contract));
    }
    count += futs.len();
    futures::future::join_all(futs).await;

    if events.page_info.has_next_page {
        (Some(events.page_info.end_cursor), count)
    } else {
        (None, count)
    }
}

async fn create_transfer(
    event: NftTransferNftTransfersEdges,
    db_client: &Client,
    contract: &Contract,
) {
    let tra = event.node.unwrap();
    let from = tra.from.unwrap_or_default().address;
    let to = tra.to.unwrap_or_default().address;

    let (nft, created) =
        GenNft::get_or_create(db_client, contract, tra.nft.nft_id.clone(), false, None).await;

    match Transfer::get_or_create(
        tra.transaction.time,
        from.clone(),
        to.clone(),
        nft._id,
        db_client,
    )
    .await
    {
        Some((_x, true)) => {
            if created {
                nft.insert(db_client, None).await;
            }

            let mut from_owner = Owner::get_or_create(db_client, from.clone(), None).await;
            let mut to_owner = Owner::get_or_create(db_client, to.clone(), None).await;
            let _ = from_owner
                .remove_owned_nft(contract.id.clone(), nft._id, db_client, None)
                .await;

            let _ = to_owner
                .add_owned_nft(nft._id, contract.id.clone(), db_client, None)
                .await;

            if to == "0x0" && !nft.burned {
                let _ = nft.burn(db_client, None).await;
            }
            if from == "0x0" && nft.burned {
                let _ = nft.mint(db_client, None).await;
            }
        }
        _ => {}
    }
}
