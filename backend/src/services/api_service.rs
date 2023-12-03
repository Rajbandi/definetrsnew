use std::sync::Arc;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use crate::{services::TokenService, models::TokenQuery};

pub struct ApiService {
    pub token_service: Arc<TokenService>,
}

impl ApiService {
    pub fn new(token_service: Arc<TokenService>) -> Self {
        ApiService {
            token_service,
        }
    }

    pub async fn get_token(data: web::Data<ApiService>, contract_address: web::Path<String>) -> impl Responder {
        match data.token_service.get_token(&contract_address).await {
            Ok(token_info) => HttpResponse::Ok().json(token_info),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }

    pub async fn get_all_tokens(data: web::Data<ApiService>) -> impl Responder {
        match data.token_service.get_latest_tokens().await {
            Ok(tokens) => HttpResponse::Ok().json(tokens),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }
    pub async fn refresh_latest_tokens(data: web::Data<ApiService>) -> impl Responder {
        match data.token_service.update_latest_tokens().await {
            Ok(_) => HttpResponse::Ok().body("Latest tokens refreshed successfully"),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
        }
    }    
}
