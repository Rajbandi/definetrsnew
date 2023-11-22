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

    
        //Please rewrite sqlx::query!() belowto use updated table above

        sqlx::query!(
            "INSERT INTO token_info (contract_address, name, symbol, decimals, total_supply, owner, is_verified, is_renounced, is_active, is_v3, is_scam, is_rug_pull, is_dump_risk,  retry_count, previous_contracts, 
                liquidity_pool_address, liqudity_period, initial_liquidity, current_liquidity, is_liquidy_locked, locked_liquidity, is_tax_modifiable, sell_tax, buy_tax, transfer_tax, score, holders_count,
                 data, code, error, date_created, date_updated) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            token_info.contract_address,
            token_info.name,
            token_info.symbol,
            token_info.decimals,
            token_info.total_supply,
            token_info.owner,
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
            token_info.error,
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
            "UPDATE token_info SET name = ?, symbol = ?, decimals = ?, total_supply = ?, owner = ?, is_verified = ?, is_renounced = ?, is_active = ?, is_v3 = ?,
            is_scam = ?, is_rug_pull = ?, is_dump_risk = ?, previous_contracts = ?, liquidity_pool_address = ?, liqudity_period = ?, initial_liquidity = ?, current_liquidity = ?,
            retry_count = ?, 
            sell_tax = ?, buy_tax = ?, transfer_tax = ?, score = ?, holders_count = ?,
            data = ?, code = ?, error = ?, date_updated = ? WHERE contract_address = ?",
            token_info.name,
            token_info.symbol,
            token_info.decimals,
            token_info.total_supply,
            token_info.owner,
            token_info.is_verified,
            token_info.is_renounced,
            token_info.is_active,
            token_info.is_v3,
            token_info.is_scam,
            token_info.is_rug_pull,
            token_info.is_dump_risk,
            token_info.previous_contracts,
            token_info.liquidity_pool_address,
            token_info.liqudity_period,
            token_info.initial_liquidity,
            token_info.current_liquidity,
            token_info.retry_count,
            token_info.sell_tax,
            token_info.buy_tax,
            token_info.transfer_tax,
            token_info.score,
            token_info.holders_count,
            token_info.data,
            token_info.code,
            token_info.error,
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
