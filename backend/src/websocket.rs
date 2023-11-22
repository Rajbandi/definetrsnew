use actix::{Actor, Addr, StreamHandler, AsyncContext, Handler, Message};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use std::{sync::{Mutex}};

lazy_static! {
   pub static ref ADMIN_CLIENTS: Mutex<Vec<Addr<WsAdminSession>>> = Mutex::new(Vec::new());
   pub static ref GENERAL_CLIENTS: Mutex<Vec<Addr<WsGeneralSession>>> = Mutex::new(Vec::new());
}

// Define message types
#[derive(Serialize, Deserialize, Clone)]
pub enum MessageType {
    Admin,
    General,
}
#[derive(Serialize, Deserialize,Clone)]
#[derive(Message)]
#[rtype(result = "()")] 
// Define a struct to represent a WebSocket message
pub struct WsMessage {
    message_type: MessageType,
    created_at: DateTime<Utc>,
    data: String,
    // additional properties can be added here
}

impl WsMessage {
    pub fn new_admin(data: String) -> Self {
        WsMessage {
            message_type: MessageType::Admin,
            created_at: Utc::now(),
            data,
        }
    }

    // Create a new message for general clients
    pub fn new_general(data: String) -> Self {
        WsMessage {
            message_type: MessageType::General,
            created_at: Utc::now(),
            data,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

pub struct WsAdminSession;

impl Actor for WsAdminSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        info!("Address {:?} started", addr.clone());
        ADMIN_CLIENTS.lock().unwrap().push(addr);
        
    }
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        let addr = ctx.address();
        let mut admin_clients = ADMIN_CLIENTS.lock().unwrap();
        if let Some(index) = admin_clients.iter().position(|a| *a == addr) {
            admin_clients.remove(index);
        }
        actix::Running::Stop
    }
    fn stopped(&mut self, ctx: &mut Self::Context) {
        // Remove from ADMIN_CLIENTS
        let addr = ctx.address();
        info!("Address {:?} stopped", addr);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsAdminSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // Handle incoming WebSocket messages here for admin clients
    }
}

// WebSocket Actor for General Clients
pub struct WsGeneralSession;

impl Actor for WsGeneralSession {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        GENERAL_CLIENTS.lock().unwrap().push(addr);
    }
    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        let addr = ctx.address();
        let mut clients = GENERAL_CLIENTS.lock().unwrap();
        if let Some(index) = clients.iter().position(|a| *a == addr) {
            clients.remove(index);
        }
        actix::Running::Stop
    }
    fn stopped(&mut self, ctx: &mut Self::Context) {
        
        let addr = ctx.address();
        info!("Address {:?} stopped", addr);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsGeneralSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // Handle incoming WebSocket messages here for general clients
    }
}

impl Handler<WsMessage> for WsAdminSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.data);
    }
}

impl Handler<WsMessage> for WsGeneralSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.data);
    }
}
pub async fn ws_admin_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    ws::start(WsAdminSession {}, &req, stream)
        .map_err(|e| e.into())
}

pub async fn ws_general_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    ws::start(WsGeneralSession {}, &req, stream)
        .map_err(|e| e.into())
}




