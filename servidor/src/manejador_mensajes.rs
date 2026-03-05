use std::sync::Arc;

use common::protocolo::{MensajesCliente, MensajesServidor};

use crate::estado_chat::EstadoChat;

//método que maneja cada posible mensaje que le puede llegar por parte del cliente
async fn procesa_mensaje(mensaje_recibido: MensajesServidor, estado: Arc<EstadoChat>) {
    match mensaje_recibido {
        MensajesServidor::Identify { username } => {
            let mut usuarios = estado.diccionario_usuarios.write().await;

            if usuarios.contains_key(&username) {
                
            }

        }
        MensajesServidor::Response { operation, result, extra } => {
            
        }
        MensajesServidor::NewUser { username } => {
            
        }
        MensajesServidor::Status { status } => {
            
        }
        MensajesServidor::NewStatus { username, status } => {
            
        }
        MensajesServidor::Users {  } => {
            
        }
        MensajesServidor::UserList { users } => {
            
        }
        MensajesServidor::Text { username, text } => {
            
        }
        MensajesServidor::TextFrom { username, text } => {
            
        }
        MensajesServidor::PublicText { text } => {
            
        }
        MensajesServidor::PublicTextFrom { username, text } => {
            
        }
        MensajesServidor::NewRoom { roomname } => {
            
        }
        MensajesServidor::Invite { roomname, usernames } => {
            
        }
        MensajesServidor::Invitation { username, roomname } => {
            
        }
        MensajesServidor::JoinRoom { roomname } => {
            
        }
        MensajesServidor::JoinedRoom { roomname, username } => {
            
        }
        MensajesServidor::RoomUsers { roomname } => {
            
        }
        MensajesServidor::RoomUserList { roomname, users } => {
            
        }
        MensajesServidor::RoomText { roomname, text } => {
            
        }
        MensajesServidor::RoomTextFrom { roomname, username, text } => {
            
        }
        MensajesServidor::LeaveRoom { roomname } => {
            
        }
        MensajesServidor::LeftRoom { roomname, username } => {
            
        }
        MensajesServidor::Disconnect {  } => {
            
        }
        MensajesServidor::Disconnected { username } => {
            
        }
    }

}
