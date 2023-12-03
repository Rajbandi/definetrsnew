use actix_web::{web, App, HttpServer};
use definetrsnew::{
    services::ApiService, clients::{DbClientPostgres, EtherscanClient}, clients::DatabaseClient,
    services::TokenService, clients::Web3Client, websocket::{ws_admin_index, ws_general_index},
};
use dotenv::dotenv;
use log::{error, info};
use std::{env, sync::Arc};

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv().ok();
    env_logger::init();

    let web3_url = std::env::var("WEB3_URL").expect("WEB3_URL must be set");
    let web3_client = Web3Client::new(&web3_url).await?;

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set for SQLite");
    let db_client: Arc<dyn DatabaseClient> = Arc::new(DbClientPostgres::new(&db_url).await.unwrap());

    info!("Starting token sync service");
    // let token_service = TokenService::new(client, db_client);
    // let arc_service = Arc::new(token_service);

    let etherscan_api_key = std::env::var("ETHERSCAN_API_KEY").expect("ETHERSCAN_API_KEY must be set");
    let etherscan_client = EtherscanClient::new(etherscan_api_key);

    let token_service = Arc::new(TokenService::new(web3_client, db_client, etherscan_client));
  
    let token_service_clone = token_service.clone();
    tokio::spawn(async move {
        match token_service_clone.sync().await {
            Ok(()) => println!("Token synchronization completed"),
            Err(e) => eprintln!("Token synchronization failed: {:?}", e),
        }
    });

    let server_binding =
        std::env::var("SERVER_BINDING").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    
    // Start the Actix web server
//    let api_service = web::Data::new(ApiService::new(token_service));
    let cloned_token_service = Arc::clone(&token_service);
    let api_service = ApiService::new(cloned_token_service);
    let api_service_data = web::Data::new(api_service);

   match HttpServer::new(move || {
        let cors = actix_cors::Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(api_service_data.clone()) 
            .route("/get_token/{contract_address}", web::get().to(ApiService::get_token))
            .route("/get_all_tokens", web::get().to(ApiService::get_all_tokens))
            .route("/refresh_latest_tokens", web::get().to(ApiService::refresh_latest_tokens))
            .route("/ws/admin", web::get().to(ws_admin_index))
            .route("/ws/updates", web::get().to(ws_general_index))
            .wrap(cors)
    })
    .bind(server_binding)? // Use the binding from the environment variable
    .run()
    .await {
        Ok(_) => info!("Server started"),
        Err(e) => error!("Server failed to start: {:?}", e),
    };

    Ok(())
    
}
