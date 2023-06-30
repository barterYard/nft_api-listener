#![allow(clippy::all, warnings)]
pub struct getCreatedContracts;
pub mod get_created_contracts {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "get_created_contracts";
    pub const QUERY : & str = "query get_created_contracts($after: ID) {\n  events(\n    typeId: \"flow.AccountContractAdded\"\n    ordering: Descending\n    after: $after\n    first: 100\n  ) {\n    edges {\n      cursor\n      node {\n        fields\n        type {\n          contract {\n            type\n          }\n          id\n          name\n        }\n      }\n    }\n    pageInfo {\n      hasNextPage\n      endCursor\n    }\n  }\n}\n" ;
    use super::*;
    use byc_helpers::mongo::mongodb::bson::Document;
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    #[allow(dead_code)]
    type Boolean = bool;
    #[allow(dead_code)]
    type Float = f64;
    #[allow(dead_code)]
    type Int = i64;
    #[allow(dead_code)]
    type ID = String;
    type JSON = Value;
    #[derive(Clone)]
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
        pub after: Option<ID>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub events: Option<GetCreatedContractsEvents>,
    }
    #[derive(Deserialize)]
    pub struct GetCreatedContractsEvents {
        pub edges: Vec<GetCreatedContractsEventsEdges>,
        #[serde(rename = "pageInfo")]
        pub page_info: GetCreatedContractsEventsPageInfo,
    }
    #[derive(Deserialize, Clone)]
    pub struct GetCreatedContractsEventsEdges {
        pub cursor: String,
        pub node: Option<GetCreatedContractsEventsEdgesNode>,
    }
    #[derive(Deserialize, Clone)]
    pub struct GetCreatedContractsEventsEdgesNode {
        pub fields: Vec<JSON>,
        #[serde(rename = "type")]
        pub type_: GetCreatedContractsEventsEdgesNodeType,
    }
    #[derive(Deserialize, Clone)]
    pub struct GetCreatedContractsEventsEdgesNodeType {
        pub contract: GetCreatedContractsEventsEdgesNodeTypeContract,
        pub id: ID,
        pub name: String,
    }
    #[derive(Deserialize, Clone)]
    pub struct GetCreatedContractsEventsEdgesNodeTypeContract {
        #[serde(rename = "type")]
        pub type_: ContractType,
    }
    #[derive(Deserialize)]
    pub struct GetCreatedContractsEventsPageInfo {
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "endCursor")]
        pub end_cursor: String,
    }
}
impl graphql_client::GraphQLQuery for getCreatedContracts {
    type Variables = get_created_contracts::Variables;
    type ResponseData = get_created_contracts::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_created_contracts::QUERY,
            operation_name: get_created_contracts::OPERATION_NAME,
        }
    }
}
