pub mod models;
use crate::listeners::flow_listener::FlowNetwork;

pub const GET_BATCH_NFT: &str = include_str!("get_batch_nft.cdc");
pub const GET_STOREFRONT_LISTING: &str = include_str!("get_storefront_listings.cdc");
pub const FLOW_CONFIG: &str = include_str!("flow.json");

pub fn get_script(script: &'static str) -> String {
    let json: serde_json::Value =
        serde_json::from_str(FLOW_CONFIG).expect("JSON was not well-formatted");
    let contracts = json.get("contracts").unwrap();
    let mut script_string = script.to_string();
    let network = FlowNetwork::get().as_str();
    let imports = script.lines().filter(|l| l.contains("import"));
    for import in imports {
        let import_line: Vec<&str> = import.trim().split(' ').collect::<Vec<&str>>();
        if import_line.len() == 4 {
            let contract = contracts.get(import_line[1]).unwrap();
            let path = import_line[3];
            let to = contract
                .get("aliases")
                .unwrap()
                .get(network)
                .unwrap()
                .as_str()
                .unwrap();
            let import_fin = match to.starts_with("0x") {
                true => import.replace(path, to),
                false => import.replace(path, &["0x", to].join("")),
            };
            script_string = script_string.replace(import, import_fin.as_str());
        }
    }
    script_string
}
