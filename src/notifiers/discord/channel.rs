use std::fmt::Display;

use crate::events::{ListingEvent, MintingEvent, TopShotEvent};
use crate::listeners::Messageable;
use byc_helpers::mongo;
use byc_helpers::mongo::models::{common::ModelCollection, mongo_doc, Nft};
use gql;
use log::error;
use serde_json::Map;
use serenity::json::{json, Value};

type ConditionFunction = Option<fn(Map<String, Value>) -> bool>;
#[derive(Debug, Clone)]
pub struct Webhook {
    pub token: String,
    pub id: u64,
    pub send_condition: ConditionFunction,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
enum NamedChannel {
    Werewolves,
    Flunks,
    Ballerz,
    Minting,
    Nfl,
    TopShot,
    #[default]
    Empty,
}

impl Display for NamedChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            NamedChannel::Werewolves => "Werewolves",
            NamedChannel::Flunks => "Flunks",
            NamedChannel::Ballerz => "Ballerz",
            NamedChannel::Nfl => "Nfl",
            NamedChannel::TopShot => "TopShot",
            NamedChannel::Minting => "Minting",
            NamedChannel::Empty => "Empty",
        };
        f.write_str(res)
    }
}

impl NamedChannel {
    fn get_channel_id(&self) -> u64 {
        match self {
            NamedChannel::Werewolves => 994_864_678_142_484_510,
            NamedChannel::Flunks => 0,
            NamedChannel::Ballerz => 0,
            NamedChannel::Nfl => 0,
            NamedChannel::TopShot => 0,
            NamedChannel::Minting => 0,
            NamedChannel::Empty => 0,
        }
    }

    fn from_string(nft_type: &str) -> NamedChannel {
        match nft_type {
            "A.28abb9f291cadaf2.BarterYardClubWerewolf.NFT" => NamedChannel::Werewolves,
            "A.e4cf4bdc1751c65d.AllDay.NFT" => NamedChannel::Nfl,
            "A.0b2a3299cc857e29.TopShot.NFT" => NamedChannel::TopShot,
            "A.329feb3ab062d289.UFC_NFT.NFT" => NamedChannel::Empty,
            "A.807c3d470888cc48.Flunks.NFT" => NamedChannel::Flunks,
            "A.e4cf4bdc1751c65d.AllDay.MomentNFTMinted" => NamedChannel::Minting,
            "gaia" => NamedChannel::Ballerz,
            _ => NamedChannel::Empty,
        }
    }

    fn can_send(&self) -> bool {
        !matches!(self, NamedChannel::Empty)
    }

    fn get_webhooks(&self) -> Vec<Webhook> {
        match self {
            NamedChannel::Nfl => vec![
                Webhook {
                    id: 997_230_029_283_004_466,
                    token: "CpYY7B9PbUVg8O9i_RVxbB8OrnjjFlLC3OfPSPfkolxqxbtHdVyQncN4bVRahEJrtubE"
                        .to_string(),
                    send_condition: Some(|_args| true),
                },
                Webhook {
                    token: "lVl6vBSe6C2Tgm50YHgLFa4-SZrArRGszwVt4aZy4TSNEG6BAVqatUt_ZtwAEjcanc-H"
                        .to_string(),
                    id: 1_006_956_948_207_439_973,
                    send_condition: Some(|_args| true),
                },
                Webhook {
                    token: "vFs5M3a60XcfXRU7-LdCuIZkusnIRK6QHVAyRY6X-fMOrc39S2oqYK_gjC9ws48YpQTS"
                        .to_string(),
                    id: 1_024_831_990_857_486_396,
                    send_condition: Some(|args| {
                        //#1s, Perfects, Jerseys
                        let tier = match args.get("tier") {
                            Some(f) => f.as_str().unwrap_or_default(),
                            _ => return false,
                        };
                        let serial = match args.get("serial") {
                            Some(f) => f.to_string().parse::<i64>().unwrap_or_default(),
                            _ => return false,
                        };
                        let max_mint_size = match args.get("max_mint_size") {
                            Some(f) => f.to_string().parse::<i64>().unwrap_or_default(),
                            _ => return false,
                        };
                        let jersey = match args.get("player_number") {
                            Some(f) => f.to_string().parse::<i64>().unwrap_or_default(),
                            _ => return false,
                        };

                        tier == "legendary"
                            || serial == 1
                            || serial == max_mint_size
                            || serial == jersey
                    }),
                },
                Webhook {
                    token: "DD5xNKSgf7KGZulp_uiybPtJDLRYrSZKH5pX4qWhNfI8QJOJHxj1AVs8g1pYrMvQBy8h"
                        .to_string(),
                    id: 1_012_410_004_877_344_848,
                    send_condition: Some(|args| {
                        //#1s, Perfects, Jerseys
                        let tier = match args.get("tier") {
                            Some(f) => f.as_str().unwrap_or_default(),
                            _ => return false,
                        };
                        let serial = match args.get("serial") {
                            Some(f) => f.to_string().parse::<i64>().unwrap_or_default(),
                            _ => return false,
                        };
                        let max_mint_size = match args.get("max_mint_size") {
                            Some(f) => f.to_string().parse::<i64>().unwrap_or_default(),
                            _ => return false,
                        };
                        let jersey = match args.get("player_number") {
                            Some(f) => f.to_string().parse::<i64>().unwrap_or_default(),
                            _ => return false,
                        };

                        tier == "legendary"
                            || serial == 1
                            || serial == max_mint_size
                            || serial == jersey
                    }),
                },
                Webhook {
                    token: "UZm6ma-jra2NhLwGYx_VUZcvEsOa26DHfoUe3B9Tw7ywXrl6-KNpVgq_-bNdlYIVLZ9o"
                        .to_string(),
                    id: 1_012_404_238_988_615_770,
                    send_condition: Some(|args| {
                        let floor = match args.get("floor") {
                            Some(f) => f.to_string().parse::<f64>().unwrap_or_default(),
                            _ => return false,
                        };
                        let price = match args.get("price") {
                            Some(f) => f.to_string().parse::<f64>().unwrap_or_default(),
                            _ => return false,
                        };
                        price < floor
                    }),
                },
                Webhook {
                    token: "am2woydsk_gGejl_kXlmurNUzT94cbhcve0JSZk5Pt0joC7bT4rKao5pDAq7fwBJJpuS"
                        .to_string(),
                    id: 1_023_291_027_919_994_942,
                    send_condition: Some(|args| {
                        let floor = match args.get("floor") {
                            Some(f) => f.to_string().parse::<f64>().unwrap_or_default(),
                            _ => return false,
                        };
                        let price = match args.get("price") {
                            Some(f) => f.to_string().parse::<f64>().unwrap_or_default(),
                            _ => return false,
                        };
                        price < floor
                    }),
                },
            ],
            NamedChannel::Minting => vec![Webhook {
                id: 1029036117720309794,
                token: "LaRXIZ13pOZwTftoECw4AAYTBWD4AsG1wte0gppc6najvB2_lk-SBrTNZl_-TAY7Zp35"
                    .to_string(),
                send_condition: Some(|_args| true),
            }],
            _ => Default::default(),
        }
    }

    fn get_channel(&self, nft_type: String) -> Channel {
        Channel {
            kind: *self,
            id: self.get_channel_id(),
            nft_type,
            webhooks: self.get_webhooks(),
            sendable: self.can_send(),
        }
    }

    async fn get_message(&self, event: &dyn Messageable) -> Option<serde_json::Value> {
        match self {
            NamedChannel::Werewolves => {
                let messageable_event = match event.as_any().downcast_ref::<ListingEvent>() {
                    Some(me) => me,
                    None => {
                        error!("Not able to dowcast event");
                        return None;
                    }
                };
                let m_client = mongo::client::create().await;
                let n_coll = Nft::get_collection(&m_client);
                let nft: Nft = match n_coll
                    .find_one(
                        mongo_doc! {"token_id": messageable_event.nft_id as i64},
                        None,
                    )
                    .await
                {
                    Ok(n) => match n {
                        Some(nf) => nf,
                        None => return None,
                    },
                    Err(err) => {
                        error!("{:?}", err);
                        drop(err);
                        return None;
                    }
                };
                Some(json!({
                    "content": format!("Werewolf #{} with score {} was listed at ${:.1}\nhttps://ongaia.com/barteryardclub/nft/{}", messageable_event.nft_id, nft.score, messageable_event.price, messageable_event.nft_id)
                }))
            }
            NamedChannel::Flunks => {
                let messageable_event = match event.as_any().downcast_ref::<ListingEvent>() {
                    Some(me) => me,
                    None => {
                        error!("Not able to dowcast event");
                        return None;
                    }
                };
                Some(json!({
                    "content":
                        format!(
                            "Flunks #{} was listed at ${:.1}$\nhttps://ongaia.com/flunks/nft/{}",
                            messageable_event.nft_id,
                            messageable_event.price,
                            messageable_event.nft_id
                        )
                }))
            }
            NamedChannel::Ballerz => {
                let messageable_event = match event.as_any().downcast_ref::<ListingEvent>() {
                    Some(me) => me,
                    None => {
                        error!("Not able to dowcast event");
                        return None;
                    }
                };
                Some(json!({
                    "content":
                        format!(
                            "nft ballerz : {} at {}",
                            messageable_event.nft_id, messageable_event.price
                        )
                }))
            }
            NamedChannel::Nfl => {
                let messageable_event = match event.as_any().downcast_ref::<ListingEvent>() {
                    Some(me) => me,
                    None => {
                        error!("Not able to dowcast event");
                        return None;
                    }
                };

                let moment = match gql::get_nfl_moment_with_retry(messageable_event.nft_id).await {
                    Ok(x) => x,
                    Err(error) => {
                        error!("{:?}", error);
                        drop(error);
                        return Some(json!({
                            "content":
                                format!(
                                    "nft NFL : {} at {} detail not found",
                                    messageable_event.nft_id, messageable_event.price
                                )
                        }));
                    }
                };

                let price_str = match messageable_event.price < moment.floor {
                    true => format!("**{:.1}**", messageable_event.price),
                    _ => format!("{:.1}", messageable_event.price),
                };

                Some(json!({
                        "floor": moment.floor,
                        "price": messageable_event.price,
                        "tier": moment.tier.as_str(),
                        "max_mint_size": moment.total_moment,
                        "serial": moment.serial_number,
                        "player_number": moment.player_number,
                        "tts": false,
                        "embeds": [{
                        "title": format!("Listing: {} {} / {}", moment.get_name(), moment.serial_number,moment.total_moment),
                        "color": moment.tier.get_color(),
                        "description": format!("Price: ${} | Floor: ${:.1}\n{}{} | {} | {} \nhttps://nflallday.com/moments/{}\nhttps://nflallday.com/listing/moment/{}/select",
                        price_str,
                        moment.floor,
                        moment.tier.as_str()[..1].to_uppercase(),
                        &(moment.tier.as_str())[1..],
                        moment.set_name,
                        moment.serie_name,
                        moment.id,
                        moment.edition_flow_id),
                    }],
                    "username": "BYC",
                }))
            }
            NamedChannel::TopShot => {
                let messageable_event = match event.as_any().downcast_ref::<TopShotEvent>() {
                    Some(me) => me.to_owned(),
                    None => {
                        let x = match event.as_any().downcast_ref::<ListingEvent>() {
                            Some(m) => {
                                let nft_id = m.nft_id;
                                let price = m.price;
                                let channel = m.channel.clone();
                                TopShotEvent {
                                    nft_id,
                                    price,
                                    seller: "".to_string(),
                                    channel,
                                }
                            }
                            None => {
                                return None;
                            }
                        };
                        x
                    }
                };
                let moment = match gql::get_topshot_moment(messageable_event.nft_id).await {
                    Ok(mo) => mo,
                    _ => {
                        error!("Not able to dowcast event");
                        return None;
                    }
                };
                Some(json!({
                    "content":
                        format!(
                            "nft Topshot : {} at ${} {}",
                            messageable_event.nft_id,
                            messageable_event.price,
                            moment.get_url()
                        )
                }))
            }
            NamedChannel::Minting => {
                let messageable_event = match event.as_any().downcast_ref::<MintingEvent>() {
                    Some(me) => me,
                    None => {
                        error!("Not able to dowcast event");
                        return None;
                    }
                };
                let moment = match gql::get_nfl_moment_with_retry(messageable_event.id).await {
                    Ok(x) => x,
                    Err(error) => {
                        error!("{:?}", error);
                        drop(error);
                        return None;
                    }
                };

                Some(json!({
                    "tts": false,
                        "embeds": [{
                    "title":
                        format!(
                            "Minting: {}  {} / {}",
                            moment.get_name(),
                            messageable_event.serial_number,
                            moment.total_moment
                        ),
                    "color": moment.tier.get_color(),
                    "content": "minting"
                    }],
                }))
            }
            NamedChannel::Empty => None,
        }
    }
}
#[derive(Debug, Clone)]
pub struct Channel {
    kind: NamedChannel,
    pub id: u64,
    pub nft_type: String,
    pub webhooks: Vec<Webhook>,
    pub sendable: bool,
}

impl Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.kind).as_str())
    }
}

impl Channel {
    pub fn get_channel_for_nft(nft: String) -> Channel {
        NamedChannel::from_string(nft.as_str()).get_channel(nft)
    }

    pub async fn get_message(&self, event: &dyn Messageable) -> Option<serde_json::Value> {
        self.kind.get_message(event).await
    }
}
