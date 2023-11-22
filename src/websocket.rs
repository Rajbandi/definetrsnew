use actix::{Actor, Addr, StreamHandler, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use log::info;
use serde::{Deserialize, Serialize};
use std::{sync::{Arc, Mutex}, error::Error};

lazy_static! {
    static ref ADMIN_CLIENTS: Mutex<Vec<Addr<WsAdminSession>>> = Mutex::new(Vec::new());
    static ref GENERAL_CLIENTS: Mutex<Vec<Addr<WsGeneralSession>>> = Mutex::new(Vec::new());
}

#[derive(Serialize, Deserialize)]
pub struct WsMessage {
    message_type: String,
    created_at: DateTime<Utc>,
    data: String,
}

impl WsMessage {
    pub fn new(message_type: String, data: String) -> Self {
        WsMessage {
            message_type,
            created_at: Utc::now(),
            data,
        }
    }

    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

struct WsAdminSession;

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
struct WsGeneralSession;

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

pub async fn ws_admin_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    ws::start(WsAdminSession {}, &req, stream)
        .map_err(|e| e.into())
}

pub async fn ws_general_index(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    ws::start(WsGeneralSession {}, &req, stream)
        .map_err(|e| e.into())
}

pub struct WsActor {
    ws_clients_manager: Arc<WsClientsManager>,
}

impl WsActor {
    pub fn new(ws_clients_manager: Arc<WsClientsManager>) -> Self {
        WsActor { ws_clients_manager }
    }
    pub fn send_json_message(
        &self,
        message_type: String,
        data: String,
        ctx: &mut ws::WebsocketContext<Self>,
    ) {
        let message = WsMessage::new(message_type, data);
        match message.to_json() {
            Ok(json_message) => ctx.text(json_message),
            Err(e) => eprintln!("Error serializing message: {}", e),
        }
    }
    pub fn handle_text_message(&self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        if let Ok(message) = serde_json::from_str::<WsMessage>(&text) {
            // Handle the message based on its type and content
        } else {
            eprintln!("Invalid JSON message received");
        }
    }
}

pub struct WsClientsManager {
    clients: Mutex<Vec<Addr<WsActor>>>,
}

impl WsClientsManager {
    pub fn new() -> Self {
        WsClientsManager {
            clients: Mutex::new(Vec::new()),
        }
    }

    pub fn add_client(&self, addr: Addr<WsActor>) {
        let mut clients = self.clients.lock().unwrap();
        clients.push(addr);
    }

    pub fn remove_client(&self, addr: &Addr<WsActor>) {
        let mut clients = self.clients.lock().unwrap();
        clients.retain(|client_addr| client_addr != addr);
    }

    pub fn send_message_to_all(&self, message: String) {
        let clients = self.clients.lock().unwrap();
        for client in clients.iter() {
            //client.do_send(WsMessage(message.clone()));
        }
    }
}

impl Actor for WsActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        // Register with the WsClientsManager
        // Assuming you have a way to access the WsClientsManager from here
        // For example, via an AppState struct or through the context
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        // Unregister from the WsClientsManager
    }
}

pub struct WebSocketActor;

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    /// Method called when a new WebSocket connection is established
    fn started(&mut self, ctx: &mut Self::Context) {
        // Handle new connection
        // For example, send a welcome message or log the connection
        ctx.text("Welcome to the WebSocket server!");
        info!("New WebSocket client connected");
    }
}

/// Handler for WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        // Existing message handling...
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }

    fn finished(&mut self, ctx: &mut Self::Context) {
        // Existing disconnection handling...
        info!("WebSocket client disconnected");
    }
}

/// WebSocket route
pub async fn websocket_route(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    info!("WebSocket route.....");
    ws::start(WebSocketActor, &req, stream)
}
