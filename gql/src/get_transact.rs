#![allow(clippy::all, warnings)]
pub struct nftTransfer;
pub mod nft_transfer {
    #![allow(dead_code)]
    use std::result::Result;
    pub const OPERATION_NAME: &str = "nftTransfer";
    pub const QUERY : & str = "query nftTransfer($contractId: ID, $after: ID) {\n  nftTransfers(\n    contractId: $contractId\n    after: $after\n    ordering: Descending\n    first: 50\n  ) {\n    edges {\n      node {\n        transaction {\n          time\n        }\n        nft {\n          nftId\n        }\n        from {\n          address\n        }\n        to {\n          address\n        }\n      }\n    }\n    pageInfo {\n      hasNextPage\n      endCursor\n    }\n  }\n}\n" ;
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
    #[derive(Serialize)]
    pub struct Variables {
        #[serde(rename = "contractId")]
        pub contract_id: Option<ID>,
        pub after: Option<ID>,
    }
    impl Variables {}
    #[derive(Deserialize)]
    pub struct ResponseData {
        #[serde(rename = "nftTransfers")]
        pub nft_transfers: NftTransferNftTransfers,
    }
    #[derive(Deserialize)]
    pub struct NftTransferNftTransfers {
        pub edges: Vec<NftTransferNftTransfersEdges>,
        #[serde(rename = "pageInfo")]
        pub page_info: NftTransferNftTransfersPageInfo,
    }
    #[derive(Deserialize)]
    pub struct NftTransferNftTransfersEdges {
        pub node: Option<NftTransferNftTransfersEdgesNode>,
    }
    #[derive(Deserialize)]
    pub struct NftTransferNftTransfersEdgesNode {
        pub transaction: NftTransferNftTransfersEdgesNodeTransaction,
        pub nft: NftTransferNftTransfersEdgesNodeNft,
        pub from: Option<NftTransferNftTransfersEdgesNodeFrom>,
        pub to: Option<NftTransferNftTransfersEdgesNodeTo>,
    }
    #[derive(Deserialize)]
    pub struct NftTransferNftTransfersEdgesNodeTransaction {
        pub time: Time,
    }
    #[derive(Deserialize)]
    pub struct NftTransferNftTransfersEdgesNodeNft {
        #[serde(rename = "nftId")]
        pub nft_id: ID,
    }
    #[derive(Deserialize)]
    pub struct NftTransferNftTransfersEdgesNodeFrom {
        pub address: ID,
    }
    #[derive(Deserialize)]
    pub struct NftTransferNftTransfersEdgesNodeTo {
        pub address: ID,
    }
    #[derive(Deserialize)]
    pub struct NftTransferNftTransfersPageInfo {
        #[serde(rename = "hasNextPage")]
        pub has_next_page: Boolean,
        #[serde(rename = "endCursor")]
        pub end_cursor: String,
    }
}
impl graphql_client::GraphQLQuery for nftTransfer {
    type Variables = nft_transfer::Variables;
    type ResponseData = nft_transfer::ResponseData;
    fn build_query(variables: Self::Variables) -> ::graphql_client::QueryBody<Self::Variables> {
        graphql_client::QueryBody {
            variables,
            query: nft_transfer::QUERY,
            operation_name: nft_transfer::OPERATION_NAME,
        }
    }
}
