
use std::{env, sync::Arc};

use definetrsnew::{web3_client::Web3Client, models::TokenInfo, dbclient::{self, DbClientSqlite}, dbtraits::DatabaseClient, token_service::TokenSyncService};
use log::{info, error};
use web3::types::{Address};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> web3::Result<()> {
    
    dotenv().ok(); 
    env_logger::init();

    let web3_url = std::env::var("WEB3_URL").expect("WEB3_URL must be set");
    let client = Web3Client::new(&web3_url).await?;
    
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set for SQLite");
    let db_client: Arc<dyn DatabaseClient> = Arc::new(DbClientSqlite::new(&db_url).await.unwrap());

    let token_service = TokenSyncService::new(client, db_client);
    let arc_service = Arc::new(token_service);

    arc_service.sync().await?;


    // let contract_address = "0xFc99C1E310758CCD0BE1866aB267C1e54Ce4f460"
    //     .parse::<Address>()
    //     .unwrap();

    // let (name, symbol, decimals, total_supply) =
    //     client.query_contract(contract_address, None).await?;
    // println!("Name: {}", name);
    // println!("Symbol: {}", symbol);
    // println!("Decimals: {}", decimals);
    // println!("Total Supply: {}", total_supply);

    // let token_info = TokenInfo {
    //     contract_address: contract_address.to_string(),
    //     name,
    //     symbol,
    //     decimals: decimals as i32,
    //     total_supply: format!("{:?}",total_supply),
    //     owner: None,
    //     is_verified: false,
    //     is_renounced: false,
    //     is_v3: false,
    //     is_active: false,
    //     info_retry_count: 0,
    //     data: None,
    //     date_created: chrono::Utc::now().naive_utc(),
    //     date_updated: None,
    // };

    // info!("Saving token info to databas {:?}", token_info);
    
    // match(db_client.save_token(&token_info).await) {
    //     Ok(_) => info!("Token info saved to database"),
    //     Err(e) => error!("Error saving token info to database: {}", e),
    // }
    
    Ok(())
}
