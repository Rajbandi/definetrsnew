use crate::websocket::{ADMIN_CLIENTS, GENERAL_CLIENTS, WsMessage};

pub struct WebSocketService;

impl WebSocketService {
    pub async fn send_to_admin_clients(message_data: &str) {
        let message = WsMessage::new_admin(message_data.to_string()); // Create your message here
        let admin_clients = ADMIN_CLIENTS.lock().unwrap();
        for client in admin_clients.iter() {
            client.do_send(message.clone());
        }
    }

    pub async fn send_to_general_clients(message_data: &str) {
        let message = WsMessage::new_general(message_data.to_string()); // Create your message here
        let general_clients = GENERAL_CLIENTS.lock().unwrap();
        for client in general_clients.iter() {
            client.do_send(message.clone());
        }
    }
}