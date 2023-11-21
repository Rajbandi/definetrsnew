use crate::{dbtraits::DatabaseClient, models::{TokenInfo, TokenQuery}};

use async_trait::async_trait;
use chrono::NaiveDateTime;
use sqlx::{Pool, Sqlite, SqlitePool};

pub struct DbClientSqlite {
    pool: Pool<Sqlite>,
        
}
impl DbClientSqlite {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = SqlitePool::connect(database_url).await?;
        Ok(DbClientSqlite { pool })
    }
}

#[async_trait]
impl DatabaseClient for DbClientSqlite {
    async fn add_token(&self, token_info: &TokenInfo) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "INSERT INTO token_info (contract_address, name, symbol, decimals, total_supply, owner, is_verified, is_renounced, is_active, info_retry_count, data, date_created, date_updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            token_info.contract_address,
            token_info.name,
            token_info.symbol,
            token_info.decimals,
            token_info.total_supply,
            token_info.owner,
            token_info.is_verified,
            token_info.is_renounced,
            token_info.is_active,
            token_info.info_retry_count,
            token_info.data,
            token_info.date_created,
            token_info.date_updated
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    //implement get_token
    async fn get_token(&self, contract_address: &str) -> Result<TokenInfo, sqlx::Error> {
        let sql_query = "SELECT * FROM token_info WHERE contract_address = ?";

        let query = TokenQuery{
            contract_address: Some(contract_address.to_string()),
            ..Default::default()
        };
        
        let mut tokens = self.get_all_tokens(query).await?;
        if tokens.len() > 0 {
            Ok(tokens.remove(0))
        } else {
            Err(sqlx::Error::RowNotFound)
        }
    }
    
    // Example: Updating a token
    async fn update_token(&self, token_info: &TokenInfo) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "UPDATE token_info SET name = ?, symbol = ?, decimals = ?, total_supply = ?, owner = ?, is_verified = ?, is_renounced = ?, is_active = ?, info_retry_count = ?, data = ?, date_updated = ? WHERE contract_address = ?",
            token_info.name,
            token_info.symbol,
            token_info.decimals,
            token_info.total_supply,
            token_info.owner,
            token_info.is_verified,
            token_info.is_renounced,
            token_info.is_active,
            token_info.info_retry_count,
            token_info.data,
            token_info.date_updated,
            token_info.contract_address
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    // Example: Deleting a token
    async fn delete_token(&self, contract_address: &str) -> Result<(), sqlx::Error> {
        sqlx::query!(
            "DELETE FROM token_info WHERE contract_address = ?",
            contract_address
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

     async fn save_token(&self, token_info: &TokenInfo) -> Result<(), sqlx::Error> {
        let  existing_token = self.get_token(&token_info.contract_address).await;

        if existing_token.is_ok() {
            let mut token  = existing_token.unwrap();
            
            token.date_updated = Some(chrono::Utc::now().naive_utc());

            if token_info.is_verified && !token.is_verified {
                token.is_verified = token_info.is_verified;
            }
            if token.is_renounced && !token.is_renounced {
                token.is_renounced = token_info.is_renounced;
            }
            if token.is_active && !token.is_active{
                token.is_active = token_info.is_active;
            }
            if token_info.info_retry_count > token.info_retry_count {
                token.info_retry_count = token_info.info_retry_count;
            }
            if token_info.owner.is_some() && token.owner.is_none() {
                token.owner = token_info.owner.clone();
            }
            if !token_info.name.is_empty() && token_info.name != token.name {
                token.name = token_info.name.clone();
            }
            if !token_info.symbol.is_empty() && token_info.symbol != token.symbol {
                token.symbol = token_info.symbol.clone();
            }
            if token_info.decimals > 0 && token_info.decimals != token.decimals {
                token.decimals = token_info.decimals;
            }
            if !token_info.total_supply.is_empty() && token_info.total_supply != token.total_supply {
                token.total_supply = token_info.total_supply.clone();
            }
            if token_info.data.is_some()
            {
                if token.data.is_none() || token_info.data != token.data {
                    token.data = token_info.data.clone();
                }
            }
            
            
            self.update_token(&token).await
        } else {
            self.add_token(token_info).await
        }
    }

    async fn get_all_tokens(&self, query: TokenQuery) -> Result<Vec<TokenInfo>, sqlx::Error> {
        let mut sql_query = "SELECT * FROM token_info WHERE 1 = 1".to_string();

        if let Some(ref name) = query.name {
            sql_query.push_str(&format!(" AND name LIKE '%{}%'", name));
        }
        if let Some(ref symbol) = query.symbol {
            sql_query.push_str(&format!(" AND symbol LIKE '%{}%'", symbol));
        }
        if let Some(ref contract_address) = query.contract_address {
            sql_query.push_str(&format!(" AND contract_address = '{}'", contract_address));
        }
        // Add other conditions similarly...

        if let (Some(from_date), Some(to_date)) = (query.from_date, query.to_date) {
            sql_query.push_str(&format!(
                " AND date_updated BETWEEN '{}' AND '{}'",
                from_date, to_date
            ));
        }

        sqlx::query_as::<_, TokenInfo>(&sql_query)
            .fetch_all(&self.pool)
            .await
    }
}
