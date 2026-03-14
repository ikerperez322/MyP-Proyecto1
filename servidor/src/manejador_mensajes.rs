use std::sync::{Arc, mpsc};
use std::collections::{HashMap, LinkedList};
use std::sync::mpsc::{Receiver, Sender};
// use common::maneja_json::deserializa_json_servidor;
use common::nombres::NombreUsuario;
use common::protocolo::{MensajesCliente, MensajesServidor};
use common::status::Status;

use crate::evento_servidor::EventoChat;
use crate::usuario;
use crate::{estado_chat::EstadoChat, usuario::Usuario};

//método que maneja cada posible mensaje que le puede llegar por parte del cliente
pub async fn procesa_mensaje(mensaje_recibido: &MensajesCliente, estado: Arc<EstadoChat>, usuario_actual: &mut Option<NombreUsuario>, sender: tokio::sync::mpsc::Sender<EventoChat>) -> Option<MensajesServidor> {
    match mensaje_recibido {
        
        MensajesCliente::Identify { username } => {

            let mut usuarios = estado.diccionario_usuarios.write().await;
            
            //usuario ya existe
            if usuarios.contains_key(&username) {
                return Some(MensajesServidor::Response { operation: ("IDENTIFY".to_string()), result: ("USER_ALREADY_EXISTS".to_string()), extra: (username.0.clone()) });
                //usuario no existe, el usuario se identifica con éxito
            }else {

                let mut mensajes_usuarios = estado.forma_mandar_mensajes.write().await;
                mensajes_usuarios.insert(username.clone(), sender);
                
                usuarios.insert(username.clone(), Usuario{
                    username: username.clone(),
                    status: Status::ACTIVE,
                    cuartos: LinkedList::new(),
                });
                *usuario_actual = Some(username.clone());

                // let msg: String = format!("{} se ha conectado", username.0);
                let msg: MensajesServidor = MensajesServidor::NewUser { username: (username.clone()) };
                
                estado.tx.send(EventoChat{
                    autor: username.clone(),
                    destino: None,
                    mensaje: msg,
                });
                
                return Some(MensajesServidor::Response { operation: ("IDENTIFY".to_string()), result: ("SUCCESS".to_string()), extra: (username.0.clone()) });
            }

        }
        MensajesCliente::Status { status } => {

            let mut usuarios = estado.diccionario_usuarios.write().await;

            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => return None,
            };

            let mut usuario_encontrado: bool = false;

            for (llave, valor) in usuarios.iter_mut() {
                if llave == usuario {
                    valor.status = status.clone();
                    usuario_encontrado = true;
                }
            }
            
            if usuario_encontrado == true {
                let msg: MensajesServidor = MensajesServidor::NewStatus {
                    username: (usuario.clone()),
                    status: (status.clone()) };
                
                estado.tx.send(EventoChat {
                    autor: (usuario.clone()),
                    destino: None,
                    mensaje: (msg.clone()) });
            
            }
                     
            return None;
        }
        MensajesCliente::Users {  } => {

            let usuarios = estado.diccionario_usuarios.read().await;

            let mut lista_usuarios: HashMap<NombreUsuario, Status> = HashMap::new();
            
            for (llave, user) in usuarios.iter() {
                lista_usuarios.insert(llave.clone(), user.status.clone());
            }
            
            return Some(MensajesServidor::UserList { users: (lista_usuarios) });
        }
        MensajesCliente::Text { username, text } => {

            let tx_destino = {
                let mapa = estado.forma_mandar_mensajes.read().await;
                mapa.get(&username).cloned()
            };

            if let Some(tx_destino) = tx_destino {

                let usuario = match usuario_actual {
                    Some(user) => user,
                    None => return None,
                };

                let msg = MensajesServidor::TextFrom {
                    username: usuario.clone(),
                    text: text.clone(),
                };

                let evento = EventoChat {
                    autor: usuario.clone(),
                    destino: Some(username.clone()),
                    mensaje: msg,
                };

                let _ = tx_destino.send(evento).await;

                return None;

            } else {

                return Some(MensajesServidor::Response {
                    operation: "TEXT".to_string(),
                    result: "NO_SUCH_USER".to_string(),
                    extra: username.0.to_string(),
                });
            }            
        }
        MensajesCliente::PublicText { text } => {

            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => return None,
            };

            
            let msg: MensajesServidor = MensajesServidor::PublicTextFrom {
                username: (usuario.clone()),
                text: (text.to_string()) };

            estado.tx.send(EventoChat {
                autor: (usuario.clone()),
                destino: None,
                mensaje: (msg.clone()) });
            
            return None;
        }
        MensajesCliente::NewRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::Invite { roomname, usernames } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::JoinRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::RoomUsers { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::RoomText { roomname, text } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::LeaveRoom { roomname } => {
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::Disconnect {  } => {

            let mut usuarios = estado.diccionario_usuarios.write().await;

            let mut mapa = estado.forma_mandar_mensajes.write().await;
            
            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => return None,
            };

            
            //borrar el usuario del diccionario
            usuarios.remove(usuario);
            mapa.remove(usuario);

            let msg: MensajesServidor = MensajesServidor::Disconnected { username: (usuario.clone()) };

            estado.tx.send(EventoChat {
                autor: (usuario.clone()),
                destino: None,
                mensaje: (msg.clone()) });
            
            return None;
        }
    }

}
