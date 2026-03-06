use std::sync::Arc;
use std::collections::LinkedList;

use common::protocolo::{MensajesCliente, MensajesServidor};
use common::status::Status;

use crate::{estado_chat::EstadoChat, usuario::Usuario};

//método que maneja cada posible mensaje que le puede llegar por parte del cliente
pub async fn procesa_mensaje(mensaje_recibido: MensajesServidor, estado: Arc<EstadoChat>) -> MensajesCliente {
    match mensaje_recibido {
        MensajesServidor::Identify { username } => {
            let mut usuarios = estado.diccionario_usuarios.write().await;

            //usuario ya existe
            if usuarios.contains_key(&username) {
                MensajesCliente::Response { operation: ("IDENTIFY".to_string()), result: ("USER_ALREADY_EXISTS".to_string()), extra: (username.0) }
                //usuario no existe, el usuario se identifica con éxito
            }else {
                usuarios.insert(username.clone(), Usuario{
                        username: username.clone(),
                        status: Status::ACTIVE,
                        cuartos: LinkedList::new(),
                    });
                MensajesCliente::Response { operation: ("IDENTIFY".to_string()), result: ("SUCCESS".to_string()), extra: (username.0) }
            }

        }
        MensajesServidor::NewUser { username } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::Status { status } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::NewStatus { username, status } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::Users {  } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::UserList { users } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::Text { username, text } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::TextFrom { username, text } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::PublicText { text } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::PublicTextFrom { username, text } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::NewRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::Invite { roomname, usernames } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::Invitation { username, roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::JoinRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::JoinedRoom { roomname, username } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::RoomUsers { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::RoomUserList { roomname, users } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::RoomText { roomname, text } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::RoomTextFrom { roomname, username, text } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::LeaveRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::LeftRoom { roomname, username } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::Disconnect {  } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesServidor::Disconnected { username } => {
            todo!("Implementar las respuestas del servidor");
        }
    }

}
