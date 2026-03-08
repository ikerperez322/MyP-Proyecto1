use std::sync::Arc;
use std::collections::LinkedList;

use common::protocolo::{MensajesCliente, MensajesServidor};
use common::status::Status;

use crate::{estado_chat::EstadoChat, usuario::Usuario};

//método que maneja cada posible mensaje que le puede llegar por parte del cliente
pub async fn procesa_mensaje(mensaje_recibido: MensajesCliente, estado: Arc<EstadoChat>) -> MensajesServidor {
    match mensaje_recibido {
        MensajesCliente::Identify { username } => {
            let mut usuarios = estado.diccionario_usuarios.write().await;

            //usuario ya existe
            if usuarios.contains_key(&username) {
                MensajesServidor::Response { operation: ("IDENTIFY".to_string()), result: ("USER_ALREADY_EXISTS".to_string()), extra: (username.0) }
                //usuario no existe, el usuario se identifica con éxito
            }else {
                usuarios.insert(username.clone(), Usuario{
                        username: username.clone(),
                        status: Status::ACTIVE,
                        cuartos: LinkedList::new(),
                    });
                MensajesServidor::Response { operation: ("IDENTIFY".to_string()), result: ("SUCCESS".to_string()), extra: (username.0) }
            }

        }
        MensajesCliente::NewUser { username } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::Status { status } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::NewStatus { username, status } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::Users {  } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::UserList { users } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::Text { username, text } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::TextFrom { username, text } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::PublicText { text } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::PublicTextFrom { username, text } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::NewRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::Invite { roomname, usernames } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::Invitation { username, roomname } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::JoinRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::JoinedRoom { roomname, username } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::RoomUsers { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::RoomUserList { roomname, users } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::RoomText { roomname, text } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::RoomTextFrom { roomname, username, text } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::LeaveRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::LeftRoom { roomname, username } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
        MensajesCliente::Disconnect {  } => {
            todo!("Implementar las respuestas del servidor");
        }
        // MensajesCliente::Disconnected { username } => {
        //     todo!("Implementar las respuestas del servidor");
        // }
    }

}
