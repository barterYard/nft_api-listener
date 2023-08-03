use std::collections::HashMap;
use std::str::FromStr;

use crate::cadence;
use crate::cadence::{models::CompositeHelper, GET_STOREFRONT_LISTING};
use crate::listeners::Cadencable;
use async_trait::async_trait;
use flow_helpers::flow_sdk;
use flow_sdk::prelude::{
    cadence_json::{AddressOwned, CompositeOwned, ValueOwned},
    TonicHyperFlowClient,
};
use log::error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StorefrontListing {
    pub price: f64,
    pub storefront_id: u64,
    pub purchased: bool,
    pub nft_type: String,
    pub nft_id: u64,
}
type StorefrontListingCompositeType = (f64, u64, bool, String, u64);

trait ToField {
    fn get_listing_field(&self) -> StorefrontListingCompositeType;
}

impl ToField for CompositeOwned {
    fn get_listing_field(&self) -> StorefrontListingCompositeType {
        let storefront_id = match Self::unwrap_optional(self, "storefrontID") {
            Some(ValueOwned::UInt64(p)) => p,
            _ => 0,
        };
        let price = match Self::unwrap_optional(self, "price") {
            Some(ValueOwned::UFix64(a)) => a.to_raw() as f64 / 100000000.0,
            _ => 0.0,
        };
        let purchased = match Self::unwrap_optional(self, "purchased") {
            Some(ValueOwned::Bool(a)) => a,
            _ => false,
        };
        let nft_type = match Self::unwrap_optional(self, "nftType") {
            Some(ValueOwned::Type(a)) => a.type_id,
            _ => "".to_string(),
        };
        let nft_id = match Self::unwrap_optional(self, "nftID") {
            Some(ValueOwned::UInt64(p)) => p,
            _ => 0,
        };
        (price, storefront_id, purchased, nft_type, nft_id)
    }
}
impl Cadencable for StorefrontListing {
    fn from_cadence(obj: &flow_sdk::prelude::cadence_json::CompositeOwned) -> Self {
        let (price, storefront_id, purchased, nft_type, nft_id) = obj.get_listing_field();
        StorefrontListing {
            price,
            storefront_id,
            purchased,
            nft_type,
            nft_id,
        }
    }
}

#[async_trait(?Send)]
trait Script {
    async fn get_listings(
        &mut self,
        address: String,
    ) -> Option<HashMap<String, Vec<StorefrontListing>>>;
}
#[async_trait(?Send)]
impl Script for TonicHyperFlowClient {
    async fn get_listings(
        &mut self,
        address: String,
    ) -> Option<HashMap<String, Vec<StorefrontListing>>> {
        let value_address = ValueOwned::Address(AddressOwned::from_str(address.as_str()).unwrap());

        let script = cadence::get_script(GET_STOREFRONT_LISTING);
        let k = match self
            .execute_script_at_latest_block(script, vec![value_address])
            .await
        {
            Ok(r) => match r.parse() {
                Ok(value) => value,
                Err(err) => {
                    error!("{:?}", err);
                    return None;
                }
            },
            Err(err) => {
                error!("{:?}", err);
                return None;
            }
        };
        let mut res: HashMap<String, Vec<StorefrontListing>> = HashMap::new();
        if let Some(ValueOwned::Dictionary(p)) = k.unwrap_optional("") {
            for entry in p {
                let key = match entry.key {
                    ValueOwned::String(k) => k,
                    _ => "".to_string(),
                };
                if let ValueOwned::Array(ar) = entry.value {
                    let mut val = Vec::new();
                    for l in ar {
                        if let Some(ValueOwned::Struct(comp)) = l.unwrap_optional("") {
                            let listing = StorefrontListing::from_cadence(&comp);
                            val.push(listing);
                        }
                    }
                    res.insert(key.clone(), val);
                }
            }
        };
        Some(res)
    }
}
