#![allow(clippy::all, warnings)]

use std::time::Duration;

use byc_helpers::mongo::{
    models::{common::ModelCollection, Contract, DateTime, Deployment},
    mongodb::{
        bson::{self, datetime::DateTimeBuilder},
        Client,
    },
};
use graphql_client::{GraphQLQuery, Response};

use crate::FLOWGRAPH_URL;

use self::get_contract::Variables;
pub struct getContract;
pub mod get_contract {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "getContract";
    pub const QUERY : & str = "query getContract($id: ID!) {\n  contract(id: $id) {\n    id\n    locked\n    deleted\n    type\n    address\n    identifier\n    deployments {\n      edges {\n        node {\n          time\n          hasError\n        }\n      }\n    }\n  }\n}\n" ;
    use super::*;
    use serde::{Deserialize, Serialize};
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type Time = String;
    #[derive(Debug)]
    pub enum ContractType {
        Default,
        FungibleToken,
        NonFungibleToken,
        Interface,
        Other(String),
    }
    impl ::serde::Serialize for ContractType {
        fn serialize<S: serde::Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
            ser.serialize_str(match *self {
                ContractType::Default => "Default",
                ContractType::FungibleToken => "FungibleToken",
                ContractType::NonFungibleToken => "NonFungibleToken",
                ContractType::Interface => "Interface",
                ContractType::Other(ref s) => &s,
            })
        }
    }
    impl<'de> ::serde::Deserialize<'de> for ContractType {
        fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
            let s: String = ::serde::Deserialize::deserialize(deserializer)?;
            match s.as_str() {
                "Default" => Ok(ContractType::Default),
                "FungibleToken" => Ok(ContractType::FungibleToken),
                "NonFungibleToken" => Ok(ContractType::NonFungibleToken),
                "Interface" => Ok(ContractType::Interface),
                _ => Ok(ContractType::Other(s)),
            }
        }
    }
    #[derive(Serialize)]
    pub struct Variables {
        pub id: ID,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub contract: Option<GetContractContract>,
    }
    #[derive(Deserialize)]
    pub struct GetContractContract {
        pub id: ID,
        pub locked: Boolean,
        pub deleted: Boolean,
        #[serde(rename = "type")]
        pub type_: ContractType,
        pub address: ID,
        pub identifier: String,
        pub deployments: Option<GetContractContractDeployments>,
    }
    #[derive(Deserialize)]
    pub struct GetContractContractDeployments {
        pub edges: Vec<GetContractContractDeploymentsEdges>,
    }
    #[derive(Deserialize)]
    pub struct GetContractContractDeploymentsEdges {
        pub node: Option<GetContractContractDeploymentsEdgesNode>,
    }
    #[derive(Deserialize)]
    pub struct GetContractContractDeploymentsEdgesNode {
        pub time: Time,
        #[serde(rename = "hasError")]
        pub has_error: Boolean,
    }
}
impl graphql_client::GraphQLQuery for getContract {
    type Variables = get_contract::Variables;
    type ResponseData = get_contract::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_contract::QUERY,
            operation_name: get_contract::OPERATION_NAME,
        }
    }
}
