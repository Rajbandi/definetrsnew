use chrono::NaiveDateTime;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Default, Debug, FromRow, Clone)]
pub struct TokenInfo {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: String,
    pub owner: Option<String>,
    pub creator: Option<String>,
    pub is_verified: bool,
    pub is_renounced: bool,
    pub is_active: bool,
    pub is_v3: bool,
    pub is_scam: bool,
    pub is_rug_pull: bool,
    pub is_dump_risk: bool,
    pub retry_count: i32,
    pub previous_contracts: i32,
    pub liquidity_pool_address: Option<String>,
    pub liqudity_period: i32,
    pub initial_liquidity: f64,
    pub current_liquidity: f64,
    pub is_liquidy_locked: bool,
    pub locked_liquidity: f64,
    pub is_tax_modifiable: bool,
    pub sell_tax: f64,
    pub buy_tax: f64,
    pub transfer_tax: f64,
    pub score: i32,
    pub holders_count: i32,
    pub data: Option<String>,
    pub code: Option<String>,
    pub abi: Option<String>,
    pub error: Option<String>,
    pub date_created: NaiveDateTime,
    pub date_updated: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct TokenQuery {
    pub name: Option<String>,
    pub symbol: Option<String>,
    pub contract_address: Option<String>,
    pub is_verified: Option<bool>,
    pub is_renounced: Option<bool>,
    pub is_active: Option<bool>,
    pub from_date: Option<NaiveDateTime>,
    pub to_date: Option<NaiveDateTime>,
    pub sort_by: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}