use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Local};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
   
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: String,
    pub total_supply: String,

    pub is_verified: bool,
    pub is_renounced: bool,
   
}

 impl TokenInfo{
    pub fn new(address: &str) -> Self {
        TokenInfo { contract_address: address.to_string(), name: "".to_string(), symbol: "".to_string(), decimals: "".to_string(), total_supply: "".to_string(), is_verified: false, is_renounced: false }
    }
}