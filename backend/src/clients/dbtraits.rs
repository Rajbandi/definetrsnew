// use async_trait::async_trait;
// use crate::models::{TokenInfo, TokenQuery};
// use sqlx::Error;

// #[async_trait]
// pub trait DatabaseClient : Send + Sync{
//     async fn add_token(&self, token_info: &TokenInfo) -> Result<(), Error>;
//     async fn get_token(&self, contract_address: &str) -> Result<TokenInfo, Error>;
//     async fn update_token(&self, token_info: &TokenInfo) -> Result<(), Error>;
//     async fn save_token(&self, token_info: &TokenInfo) -> Result<(), Error>;
//     async fn delete_token(&self, contract_address: &str) -> Result<(), Error>;
//     async fn get_all_tokens(&self, query: TokenQuery) -> Result<Vec<TokenInfo>, sqlx::Error>;
    
// }

use sqlx::{Database, Error as SqlxError};
use async_trait::async_trait;

use crate::models::{TokenInfo, TokenQuery};

#[async_trait]
pub trait DatabaseClient: Send + Sync {
    async fn add_token(&self, token_info: &TokenInfo) -> Result<(), SqlxError>;
    async fn get_token(&self, contract_address: &str) -> Result<TokenInfo, SqlxError>;
    async fn update_token(&self, token_info: &TokenInfo) -> Result<(), SqlxError>;
    async fn delete_token(&self, contract_address: &str) -> Result<(), SqlxError>;
    async fn get_all_tokens(&self, query: &TokenQuery) -> Result<Vec<TokenInfo>, SqlxError>;
    async fn save_token(&self, token_info: &TokenInfo) -> Result<(), SqlxError>;
    
}
