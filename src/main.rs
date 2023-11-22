use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use definetrsnew::{
    api_service::ApiService, dbclient::DbClientSqlite, dbtraits::DatabaseClient,
    token_service::TokenService, web3_client::Web3Client, websocket::{websocket_route, ws_admin_index, ws_general_index},
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
    let db_client: Arc<dyn DatabaseClient> = Arc::new(DbClientSqlite::new(&db_url).await.unwrap());

    info!("Starting token sync service");
    // let token_service = TokenService::new(client, db_client);
    // let arc_service = Arc::new(token_service);

    let token_service = Arc::new(TokenService::new(web3_client, db_client));
  

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
        App::new()
            .app_data(api_service_data.clone()) 
            .route("/get_token/{contract_address}", web::get().to(ApiService::get_token))
            .route("/get_all_tokens", web::get().to(ApiService::get_all_tokens))
            .route("/ws/admin", web::get().to(ws_admin_index))
            .route("/ws/updates", web::get().to(ws_general_index))
    })
    .bind(server_binding)? // Use the binding from the environment variable
    .run()
    .await {
        Ok(_) => info!("Server started"),
        Err(e) => error!("Server failed to start: {:?}", e),
    };

    Ok(())
    
}
