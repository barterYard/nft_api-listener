use std::error::Error;
use std::fmt;
use std::time::Duration;

use byc_helpers::mongo::{
    models::{common::ModelCollection, Contract, DateTime, Deployment, GenNft, Owner, Transfert},
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
use log::error;

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
pub async fn find_contract(contract_id: String, db_client: &Client) {
    let variables = crate::get_contract::get_contract::Variables {
        id: contract_id.clone(),
    };
    let query = getContract::build_query(variables);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(4))
        .build()
        .unwrap();
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
            let mut deps: Vec<Deployment> = vec![];
            for dep in contract.deployments.unwrap().edges {
                deps.push(Deployment {
                    time: DateTime::parse_rfc3339_str(dep.node.unwrap().time).unwrap(),
                })
            }
            //save to db
            let db_ctr = Contract {
                _id: bson::oid::ObjectId::new(),
                address: contract.address,
                id: contract.id,
                locked: contract.locked,
                deleted: contract.deleted,
                identifier: contract.identifier,
                contract_type: format!("{:?}", contract.type_),
                deployments: deps,
            };

            let contract_col = Contract::get_collection(db_client);
            let res = contract_col.insert_one(db_ctr, None).await;
            if res.is_err() {
                println!("{}", res.unwrap_err());
            } else {
                println!("contract {} added!!", contract_id);
            }
        }

        _ => {}
    };
}

pub async fn find_created_events(
    after: Option<String>,
    db_client: &Client,
    db_contract: &mut Vec<String>,
) -> Option<String> {
    let mut c = after;
    if c.clone().unwrap_or("".to_string()) == "" {
        c = None;
    }

    let variables = get_created_contract::get_created_contracts::Variables { after: c.clone() };

    let query = getCreatedContracts::build_query(variables);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(4))
        .build()
        .unwrap();

    let res = match client.post(FLOWGRAPH_URL).json(&query).send().await {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            return c;
        }
    };
    let response_body: Response<<getCreatedContracts as GraphQLQuery>::ResponseData> =
        res.json().await.unwrap();

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
        if !db_contract.contains(&&contract_id.to_string()) {
            db_contract.push(contract_id.clone());
            find_contract(contract_id, db_client).await;
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
    db_client: &Client,
) -> Option<String> {
    let t_id = Some(contract_id.clone() + &ev.clone());
    println!("{:?}", t_id);
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
        println!("{:?}", event.node.unwrap().fields);
    }
    if events.page_info.has_next_page {
        Some(events.page_info.end_cursor)
    } else {
        None
    }
}

pub async fn find_all_transactions(
    contract: Contract,
    contract_id: String,
    after: Option<String>,
    db_client: &Client,
) -> Option<String> {
    let mut c = after;
    if c.clone().unwrap_or("".to_string()) == "" {
        c = None;
    }
    let variables = get_transact::nft_transfer::Variables {
        after: c.clone(),
        contract_id: Some(contract_id),
    };
    let query = nftTransfer::build_query(variables);
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(4))
        .build()
        .unwrap();
    let res = match client.post(FLOWGRAPH_URL).json(&query).send().await {
        Ok(x) => x,
        Err(e) => {
            println!("{:?}", e);
            return c;
        }
    };
    let response_body: Response<<nftTransfer as GraphQLQuery>::ResponseData> =
        res.json().await.unwrap();
    let events = response_body.data.unwrap().nft_transfers;
    for event in events.edges {
        let tra = event.node.unwrap();
        let from = match tra.from {
            Some(x) => x.address,
            _ => "0x0".to_string(),
        };
        let to = match tra.to {
            Some(x) => x.address,
            _ => "0x0".to_string(),
        };
        let nft = GenNft::get_or_create(db_client, contract._id, tra.nft.nft_id.clone()).await;

        let mut from_owner = Owner::get_or_create(db_client, from.clone()).await;
        let mut to_owner = Owner::get_or_create(db_client, to.clone()).await;
        if to == "0x0" && !nft.burned {
            nft.burn(db_client).await;
        }
        if from == "0x0" && nft.burned {
            nft.mint(db_client).await;
        }
        from_owner.remove_owned_nft(nft._id, db_client).await;
        to_owner.add_owned_nft(nft._id, db_client).await;
        match Transfert::create(
            tra.transaction.time,
            from.clone(),
            to.clone(),
            nft._id,
            db_client,
        )
        .await
        {
            Ok(_x) => {}
            Err(e) => {
                error!("{:?}", e);
            }
        };
        // println!("transfert of {} from {} to {} done", nft._id, from, to)
        // transfer
    }
    if events.page_info.has_next_page {
        Some(events.page_info.end_cursor)
    } else {
        None
    }
}
