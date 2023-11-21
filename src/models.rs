use chrono::NaiveDateTime;
use chrono::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[serde(rename_all = "camelCase")]
#[derive(Serialize, Deserialize, Default, Debug, FromRow)]
pub struct TokenInfo {
    pub contract_address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: i32,
    pub total_supply: String,
    pub owner: Option<String>,
    pub is_verified: bool,
    pub is_renounced: bool,
    pub is_active: bool,
    pub is_v3: bool,
    pub info_retry_count: i32,
    pub data: Option<serde_json::Value>,
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
}