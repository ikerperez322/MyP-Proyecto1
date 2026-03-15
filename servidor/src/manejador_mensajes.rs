use std::sync::Arc;
use std::collections::{HashMap, LinkedList};
use common::nombres::NombreUsuario;
use common::protocolo::{MensajesCliente, MensajesServidor};
use common::status::Status;
use tokio::sync::mpsc::Sender;

use crate::evento_servidor::EventoChat;
use crate::{estado_chat::EstadoChat, usuario::Usuario};

//método que maneja cada posible mensaje que le puede llegar por parte del cliente
pub async fn procesa_mensaje(mensaje_recibido: &MensajesCliente, estado: Arc<EstadoChat>, usuario_actual: &mut Option<NombreUsuario>, sender: tokio::sync::mpsc::Sender<EventoChat>) -> Option<MensajesServidor> {
    
    match mensaje_recibido {
        MensajesCliente::Identify { username } => {

            let mut usuarios = estado.diccionario_usuarios.write().await;
            
            //checamos que el cliente no se quiera volver a identificar desde la misma dirección
            if usuario_actual.is_some() {
                return None;
                //usuario ya existe
            } else if usuarios.contains_key(&username) {
                return Some(MensajesServidor::Response { operation: ("IDENTIFY".to_string()), result: ("USER_ALREADY_EXISTS".to_string()), extra: (Some(username.0.clone())) });
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

                envia_mensajes_secundarios_publicos(username.clone(), None,
                    MensajesServidor::NewUser {
                    username: (username.clone()) },
                    estado.clone());
                
                return Some(MensajesServidor::Response { operation: ("IDENTIFY".to_string()), result: ("SUCCESS".to_string()), extra: (Some(username.0.clone())) });
            }

        }
        MensajesCliente::Status { status } => {

            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };            
            
            let mut usuarios = estado.diccionario_usuarios.write().await;

            let mut usuario_encontrado: bool = false;

            for (llave, user) in usuarios.iter_mut() {
                if llave == usuario {
                    user.status = status.clone();
                    usuario_encontrado = true;
                }
            }
            
            if usuario_encontrado == true {
                envia_mensajes_secundarios_publicos(usuario.clone(), None,
                    MensajesServidor::NewStatus {
                        username: (usuario.clone()),
                        status: (status.clone()) },
                    estado.clone());
                
            }                     
            return None;
        }
        MensajesCliente::Users {  } => {

            if usuario_actual.is_none() {
                return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
            }
            
            let usuarios = estado.diccionario_usuarios.read().await;

            let mut lista_usuarios: HashMap<NombreUsuario, Status> = HashMap::new();
            
            for (llave, user) in usuarios.iter() {
                lista_usuarios.insert(llave.clone(), user.status.clone());
            }
            
            return Some(MensajesServidor::UserList {
                users: (lista_usuarios)
            });
        }
        MensajesCliente::Text { username, text } => {

            let usuario = match usuario_actual {
                    Some(user) => user,
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };
            
            let tx_destino = {
                let mapa = estado.forma_mandar_mensajes.read().await;
                mapa.get(&username).cloned()
            };

            if let Some(tx_destino) = tx_destino {

                envia_mensajes_secundarios_privados(usuario.clone(), Some(username.clone()), MensajesServidor::TextFrom {
                    username: (usuario.clone()),
                    text: (text.clone()) },
                    tx_destino).await;

                return None;

            } else {
                return Some(MensajesServidor::Response {
                    operation: "TEXT".to_string(),
                    result: "NO_SUCH_USER".to_string(),
                    extra: Some(username.0.to_string()),
                });
            }            
        }
        MensajesCliente::PublicText { text } => {

            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };

            envia_mensajes_secundarios_publicos(usuario.clone(), None,
                MensajesServidor::PublicTextFrom {
                    username: (usuario.clone()),
                    text: (text.to_string()) },
                estado);
            
            return None;
        }
        MensajesCliente::NewRoom { roomname } => {
            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::Invite { roomname, usernames } => {
            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::JoinRoom { roomname } => {
            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::RoomUsers { roomname } => {
            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::RoomText { roomname, text } => {
            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::LeaveRoom { roomname } => {
            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };
            todo!("Implementar las respuestas del servidor");
        }
        MensajesCliente::Disconnect {  } => {

            let mut usuarios = estado.diccionario_usuarios.write().await;

            let mut mapa = estado.forma_mandar_mensajes.write().await;
            
            let usuario = match usuario_actual {
                Some(user) => user,
                //arreglar esto
                None => {
                    return Some(MensajesServidor::Response{
                        operation: ("INVALID".to_string()),
                        result: ("NOT_IDENTIFIED".to_string()),
                        extra: (None) });
                },
            };

            
            //borrar el usuario del diccionario
            usuarios.remove(usuario);
            mapa.remove(usuario);

            envia_mensajes_secundarios_publicos(usuario.clone(), None,
                MensajesServidor::Disconnected {
                    username: (usuario.clone()) },
                estado.clone());

            
            return None;
        }
    }

}

//envia los mensajes a los demás usuarios conectados con tokio broadcast
fn envia_mensajes_secundarios_publicos(autor: NombreUsuario, destino: Option<NombreUsuario>, mensaje: MensajesServidor, estado: Arc<EstadoChat>) {

    let _ = estado.tx.send(EventoChat{
        autor: autor,
        destino: destino,
        mensaje: mensaje,
    });
}

async fn envia_mensajes_secundarios_privados(autor: NombreUsuario, destino: Option<NombreUsuario>, mensaje: MensajesServidor, tx_destino: Sender<EventoChat>) {

    let _ = tx_destino.send(EventoChat{
        autor: autor,
        destino: destino,
        mensaje: mensaje,
    }).await;
    
}
