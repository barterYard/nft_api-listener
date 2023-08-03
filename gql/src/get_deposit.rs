#![allow(clippy::all, warnings)]

use std::time::Duration;

use flow_helpers::mongo::mongodb::Client;
use graphql_client::{GraphQLQuery, Response};

use crate::FLOWGRAPH_URL;

use self::get_deposit_event::Variables;
pub struct getDepositEvent;
pub mod get_deposit_event {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "get_deposit_event";
    pub const QUERY : & str = "query get_deposit_event($typeId: String, $after: ID) {\n  events(typeId: $typeId, first: 10, after: $after, ordering: Ascending) {\n    edges {\n      cursor\n      node {\n        fields\n      }\n    }\n    pageInfo {\n      hasNextPage\n      endCursor\n    }\n  }\n}\n" ;
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
    type JSON = serde_json::Value;
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "typeId")]
        pub type_id: Option<String>,
        pub after: Option<ID>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        pub events: Option<GetDepositEventEvents>,
    }
    #[derive(Deserialize)]
    pub struct GetDepositEventEvents {
        pub edges: Vec<GetDepositEventEventsEdges>,
        #[serde(rename = "pageInfo")]
        pub page_info: GetDepositEventEventsPageInfo,
    }
    #[derive(Deserialize)]
    pub struct GetDepositEventEventsEdges {
        pub cursor: String,
        pub node: Option<GetDepositEventEventsEdgesNode>,
    }
    #[derive(Deserialize)]
    pub struct GetDepositEventEventsEdgesNode {
        pub fields: Vec<JSON>,
    }
    #[derive(Deserialize)]
    pub struct GetDepositEventEventsPageInfo {
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "endCursor")]
        pub end_cursor: String,
    }
}
impl graphql_client::GraphQLQuery for getDepositEvent {
    type Variables = get_deposit_event::Variables;
    type ResponseData = get_deposit_event::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: get_deposit_event::QUERY,
            operation_name: get_deposit_event::OPERATION_NAME,
        }
    }
}
