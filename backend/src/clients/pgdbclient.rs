use sqlx::{Pool, Postgres, postgres::PgPool, Error as SqlxError};
use async_trait::async_trait;
use serde_json::Value as JsonValue;
use chrono::NaiveDateTime;


use crate::models::{TokenInfo, TokenQuery};

use super::DatabaseClient;

// Assuming TokenInfo and TokenQuery are already defined
// ...

pub struct DbClientPostgres {
    pool: Pool<Postgres>,
}

impl DbClientPostgres {
    pub async fn new(database_url: &str) -> Result<Self, SqlxError> {
        let pool = PgPool::connect(database_url).await?;
        Ok(DbClientPostgres { pool })
    }
}

#[async_trait]
impl DatabaseClient for DbClientPostgres {
    async fn add_token(&self, token_info: &TokenInfo) -> Result<(), SqlxError> {
        sqlx::query!(
            "INSERT INTO token_info (contract_address, name, symbol, decimals, total_supply, owner, creator, is_verified, is_renounced, is_active, is_v3, is_scam, is_rug_pull, is_dump_risk, retry_count, previous_contracts, liquidity_pool_address, liqudity_period, initial_liquidity, current_liquidity, is_liquidy_locked, locked_liquidity, is_tax_modifiable, sell_tax, buy_tax, transfer_tax, score, holders_count, data, code, abi, error, date_created, date_updated) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22, $23, $24, $25, $26, $27, $28, $29, $30, $31, $32, $33, $34)",
            token_info.contract_address,
            token_info.name,
            token_info.symbol,
            token_info.decimals,
            token_info.total_supply,
            token_info.owner,
            token_info.creator,
            token_info.is_verified,
            token_info.is_renounced,
            token_info.is_active,
            token_info.is_v3,
            token_info.is_scam,
            token_info.is_rug_pull,
            token_info.is_dump_risk,
            token_info.retry_count,
            token_info.previous_contracts,
            token_info.liquidity_pool_address,
            token_info.liqudity_period,
            token_info.initial_liquidity,
            token_info.current_liquidity,
            token_info.is_liquidy_locked,
            token_info.locked_liquidity,
            token_info.is_tax_modifiable,
            token_info.sell_tax,
            token_info.buy_tax,
            token_info.transfer_tax,
            token_info.score,
            token_info.holders_count,
            token_info.data,
            token_info.code,
            token_info.abi,
            token_info.error,
            token_info.date_created,
            token_info.date_updated
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_token(&self, contract_address: &str) -> Result<TokenInfo, SqlxError> {
        let token = sqlx::query_as!(TokenInfo,
            "SELECT * FROM token_info WHERE contract_address = $1",
            contract_address
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(token)
    }

    async fn update_token(&self, token_info: &TokenInfo) -> Result<(), SqlxError> {
        sqlx::query!(
            "UPDATE token_info SET name = $1, symbol = $2, decimals = $3, total_supply = $4, owner = $5, creator = $6, is_verified = $7, is_renounced = $8, is_active = $9, is_v3 = $10, is_scam = $11, is_rug_pull = $12, is_dump_risk = $13, retry_count = $14, previous_contracts = $15, liquidity_pool_address = $16, liqudity_period = $17, initial_liquidity = $18, current_liquidity = $19, is_liquidy_locked = $20, locked_liquidity = $21, is_tax_modifiable = $22, sell_tax = $23, buy_tax = $24, transfer_tax = $25, score = $26, holders_count = $27, data = $28, code = $29, abi = $30, error = $31, date_updated = $32 WHERE contract_address = $33",
            token_info.name,
            token_info.symbol,
            token_info.decimals,
            token_info.total_supply,
            token_info.owner,
            token_info.creator,
            token_info.is_verified,
            token_info.is_renounced,
            token_info.is_active,
            token_info.is_v3,
            token_info.is_scam,
            token_info.is_rug_pull,
            token_info.is_dump_risk,
            token_info.retry_count,
            token_info.previous_contracts,
            token_info.liquidity_pool_address,
            token_info.liqudity_period,
            token_info.initial_liquidity,
            token_info.current_liquidity,
            token_info.is_liquidy_locked,
            token_info.locked_liquidity,
            token_info.is_tax_modifiable,
            token_info.sell_tax,
            token_info.buy_tax,
            token_info.transfer_tax,
            token_info.score,
            token_info.holders_count,
            token_info.data,
            token_info.code,
            token_info.abi,
            token_info.error,
            token_info.date_updated,
            token_info.contract_address
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete_token(&self, contract_address: &str) -> Result<(), SqlxError> {
        sqlx::query!("DELETE FROM token_info WHERE contract_address = $1", contract_address)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_all_tokens(&self, token_query: &TokenQuery) -> Result<Vec<TokenInfo>, sqlx::Error> {
        let mut sql = "SELECT * FROM token_info".to_string();
        let mut conditions = Vec::new();
        let mut index = 1;

        if let Some(ref name) = token_query.name {
            conditions.push(format!("name = ${}", index));
            index += 1;
        }
        if let Some(ref symbol) = token_query.symbol {
            conditions.push(format!("symbol = ${}", index));
            index += 1;
        }
        if let Some(is_verified) = token_query.is_verified {
            conditions.push(format!("is_verified = ${}", index));
            index += 1;
        }
        if let Some(is_renounced) = token_query.is_renounced {
            conditions.push(format!("is_renounced = ${}", index));
            index += 1;
        }
        if let Some(is_active) = token_query.is_active {
            conditions.push(format!("is_active = ${}", index));
            index += 1;
        }
        if let Some(ref from_date) = token_query.from_date {
            conditions.push(format!("date_created >= ${}", index));
            index += 1;
        }
        if let Some(ref to_date) = token_query.to_date {
            conditions.push(format!("date_created <= ${}", index));
            index += 1;
        }
        if let Some(ref contract_address) = token_query.contract_address {
            conditions.push(format!("contract_address = ${}", index));
            index += 1;
        }

        if !conditions.is_empty() {
            sql.push_str(" WHERE ");
            sql.push_str(&conditions.join(" AND "));
        }

       // Controlled Sorting Logic
        match token_query.sort_by.as_deref() {
            Some("name_asc") => sql.push_str(" ORDER BY name ASC"),
            Some("name_desc") => sql.push_str(" ORDER BY name DESC"),
            Some("date_created_asc") => sql.push_str(" ORDER BY date_created ASC"),
            Some("date_created_desc") => sql.push_str(" ORDER BY date_created DESC"),
            _ => (), // default case or throw an error
        }

        if let Some(limit) = token_query.limit {
            sql.push_str(&format!(" LIMIT ${}", index));
            index += 1;
        }

        if let Some(offset) = token_query.offset {
            sql.push_str(&format!(" OFFSET ${}", index));
        }

        let mut query = sqlx::query_as::<_, TokenInfo>(&sql);

        if let Some(ref name) = token_query.name {
            query = query.bind(name);
        }
        if let Some(ref symbol) = token_query.symbol {
            query = query.bind(symbol);
        }
        if let Some(is_verified) = token_query.is_verified {
            query = query.bind(is_verified);
        }
        if let Some(is_renounced) = token_query.is_renounced {
            query = query.bind(is_renounced);
        }
        if let Some(is_active) = token_query.is_active {
            query = query.bind(is_active);
        }
        if let Some(ref from_date) = token_query.from_date {
            query = query.bind(from_date);
        }
        if let Some(ref to_date) = token_query.to_date {
            query = query.bind(to_date);
        }
        if let Some(ref contract_address) = token_query.contract_address {
            query = query.bind(contract_address);
        }
        if let Some(limit) = token_query.limit {
            query = query.bind(limit as i64);
        }
        if let Some(offset) = token_query.offset {
            query = query.bind(offset as i64);
        }

        let tokens = query.fetch_all(&self.pool).await?;
        Ok(tokens)
    }
        // async fn get_all_tokens(&self, query: &TokenQuery) -> Result<Vec<TokenInfo>, sqlx::Error>{
    //     // Implement logic to handle dynamic querying based on TokenQuery
    //     // For simplicity, the following is a basic implementation:
    //     let mut sql = "SELECT * FROM token_info".to_string();
    //     // Logic to append conditions based on TokenQuery fields should be added here
    //     // ...

    //     let tokens = sqlx::query_as::<_, TokenInfo>(&sql)
    //         .fetch_all(&self.pool)
    //         .await?;
    //     Ok(tokens)
    // }
    async fn save_token(&self, token_info: &TokenInfo) -> Result<(), sqlx::Error> {
        let  existing_token = self.get_token(&token_info.contract_address).await;

        if existing_token.is_ok() {
            let mut token  = existing_token.unwrap();
            
            token.date_updated = Some(chrono::Utc::now().naive_utc());

            if token_info.is_verified && !token.is_verified {
                token.is_verified = token_info.is_verified;
            }
            if token_info.is_renounced && !token.is_renounced {
                token.is_renounced = token_info.is_renounced;
            }
            if token_info.is_active && !token.is_active{
                token.is_active = token_info.is_active;
            }
            if token_info.retry_count > token.retry_count {
                token.retry_count = token_info.retry_count;
            }
            if token_info.owner.is_some() && token.owner.is_none() {
                token.owner = token_info.owner.clone();
            }
            if token_info.creator.is_some() && token.creator.is_none() {
                token.creator = token_info.creator.clone();
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
            if token_info.is_v3 && !token.is_v3 {
                token.is_v3 = token_info.is_v3;
            }
            if token_info.is_scam && !token.is_scam {
                token.is_scam = token_info.is_scam;
            }
            if token_info.is_rug_pull && !token.is_rug_pull {
                token.is_rug_pull = token_info.is_rug_pull;
            }
            if token_info.is_dump_risk && !token.is_dump_risk {
                token.is_dump_risk = token_info.is_dump_risk;
            }
            if token_info.previous_contracts > 0 && token_info.previous_contracts != token.previous_contracts {
                token.previous_contracts = token_info.previous_contracts;
            }
            if token_info.liquidity_pool_address.is_some() && token.liquidity_pool_address.is_none() {
                token.liquidity_pool_address = token_info.liquidity_pool_address.clone();
            }
            if token_info.liqudity_period > 0 && token_info.liqudity_period != token.liqudity_period {
                token.liqudity_period = token_info.liqudity_period;
            }
            if token_info.initial_liquidity > 0.0 && token_info.initial_liquidity != token.initial_liquidity {
                token.initial_liquidity = token_info.initial_liquidity;
            }
            if token_info.current_liquidity > 0.0 && token_info.current_liquidity != token.current_liquidity {
                token.current_liquidity = token_info.current_liquidity;
            }
            if token_info.is_liquidy_locked && !token.is_liquidy_locked {
                token.is_liquidy_locked = token_info.is_liquidy_locked;
            }
            if token_info.locked_liquidity > 0.0 && token_info.locked_liquidity != token.locked_liquidity {
                token.locked_liquidity = token_info.locked_liquidity;
            }
            if token_info.is_tax_modifiable && !token.is_tax_modifiable {
                token.is_tax_modifiable = token_info.is_tax_modifiable;
            }
            if token_info.sell_tax > 0.0 && token_info.sell_tax != token.sell_tax {
                token.sell_tax = token_info.sell_tax;
            }
            if token_info.buy_tax > 0.0 && token_info.buy_tax != token.buy_tax {
                token.buy_tax = token_info.buy_tax;
            }
            if token_info.transfer_tax > 0.0 && token_info.transfer_tax != token.transfer_tax {
                token.transfer_tax = token_info.transfer_tax;
            }
            if token_info.score > 0 && token_info.score != token.score {
                token.score = token_info.score;
            }
            if token_info.holders_count > 0 && token_info.holders_count != token.holders_count {
                token.holders_count = token_info.holders_count;
            }
            if token_info.data.is_some()
            {
                if token.data.is_none() || token_info.data != token.data {
                    token.data = token_info.data.clone();
                }
            }
            
            if token_info.code.is_some() {
                if token.code.is_none() || token_info.code != token.code {
                    token.code = token_info.code.clone();
                }
            }
            
            if token_info.abi.is_some() {
                if token.abi.is_none() || token_info.abi != token.abi {
                    token.abi = token_info.abi.clone();
                }
            }
            
            if token_info.error.is_some() {
                if token.error.is_none() || token_info.error != token.error {
                    token.error = token_info.error.clone();
                }
            }

            self.update_token(&token).await
        } else {
            self.add_token(token_info).await
        }
    }

}
