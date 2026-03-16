use std::sync::Arc;
use std::collections::{HashMap, LinkedList};
use common::nombres::NombreUsuario;
use common::protocolo::{MensajesCliente, MensajesServidor};
use common::status::Status;
use tokio::sync::RwLock;
use tokio::sync::mpsc::Sender;

use crate::cuarto::Cuarto;
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
            }
            //usuario ya existe
            if usuarios.contains_key(&username) {
                return Some(MensajesServidor::Response {
                    operation: ("IDENTIFY".to_string()),
                    result: ("USER_ALREADY_EXISTS".to_string()),
                    extra: (Some(username.0.clone())) });
                //usuario no existe, el usuario se identifica con éxito
            }
            // {

            let mut mensajes_usuarios = estado.forma_mandar_mensajes.write().await;
            mensajes_usuarios.insert(username.clone(), sender);
                
            usuarios.insert(username.clone(), Arc::new(RwLock::new(Usuario{
                username: username.clone(),
                status: Status::ACTIVE,
                cuartos: LinkedList::new(),
            })));
            *usuario_actual = Some(username.clone());

            envia_mensajes_secundarios_publicos(username.clone(), None,
                MensajesServidor::NewUser {
                    username: (username.clone()) },
                estado.clone());
                
            return Some(MensajesServidor::Response {
                operation: ("IDENTIFY".to_string()),
                result: ("SUCCESS".to_string()),
                extra: (Some(username.0.clone())) });
            // }

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
            
            let usuarios = estado.diccionario_usuarios.read().await;

            let instancia_usuario = match usuarios.get(&usuario) {
                Some(usr) => Arc::clone(usr),
                None => return None,
            };

            drop(usuarios);

            {
                let mut user_lock = instancia_usuario.write().await;
                user_lock.status = status.clone();
            }

            envia_mensajes_secundarios_publicos(usuario.clone(),
                None,
                MensajesServidor::NewStatus {
                    username: (usuario.clone()),
                    status: (status.clone()) },
                estado.clone());
            
            // let mut usuario_encontrado: bool = false;
            
            // for (llave, user) in usuarios.iter_mut() {
            //     if llave == usuario {
            //         user.status = status.clone();
            //         usuario_encontrado = true;
            //     }
            // }
            
            // if usuario_encontrado == true {
            //     envia_mensajes_secundarios_publicos(usuario.clone(), None,
            //         MensajesServidor::NewStatus {
            //             username: (usuario.clone()),
            //             status: (status.clone()) },
            //         estado.clone());
                
            // }          
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
                let user_arc = user.read().await;
                lista_usuarios.insert(llave.clone(), user_arc.status.clone());
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

            let mut cuartos = estado.diccionario_cuartos.write().await;

            if cuartos.contains_key(&roomname) {
                return Some(MensajesServidor::Response {
                    operation: ("NEW_ROOM".to_string()),
                    result: ("ROOM_ALREADY_EXISTS".to_string()),
                    extra: (Some(roomname.0.clone())) });
            }

            let usuarios = estado.diccionario_usuarios.read().await;

            //una instancia del usuario actual para agregar el cuarto a la lista de cuartos donde el usuario anda metido
            let instancia_usuario = match usuarios.get(&usuario) {
                Some(usr) => Arc::clone(usr),
                None => return None,
            };

            drop(usuarios);

            let mut lista = LinkedList::new();
            lista.push_back(Arc::clone(&instancia_usuario));
            
            let cuarto_nuevo = Arc::new( RwLock::new(Cuarto {
                nombre: roomname.clone(),
                lista_usuarios: lista}));
            
            {
                let mut usuario_lock = instancia_usuario.write().await;
                usuario_lock.cuartos.push_back(Arc::clone(&cuarto_nuevo));
            }

            cuartos.insert(roomname.clone(), Arc::clone(&cuarto_nuevo));
            
            return Some(MensajesServidor::Response {
                operation: ("NEW_ROOM".to_string()),
                result: ("SUCCESS".to_string()),
                extra: (Some(roomname.0.clone())) });
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

            let cuartos = estado.diccionario_cuartos.read().await;

            //el cuarto no existe
            if !cuartos.contains_key(&roomname) {
                return Some(MensajesServidor::Response {
                    operation: ("INVITE".to_string()),
                    result: ("NO_SUCH_ROOM".to_string()),
                    extra: (Some(roomname.0.clone())) });
            }
            
            let usuarios = estado.diccionario_usuarios.read().await;

            //uno o más usuarios no existen
            for usr in usernames.iter() {
                if !usuarios.contains_key(usr) {
                    return Some(MensajesServidor::Response {
                        operation: ("INVITE".to_string()),
                        result: ("NO_SUCH_USER".to_string()),
                        extra: (Some(usr.0.clone())) });
                }
                let tx_destino = {
                    let mapa = estado.forma_mandar_mensajes.read().await;
                    mapa.get(usr).cloned()
                };

                if let Some(tx_destino) = tx_destino {
                    envia_mensajes_secundarios_privados(
                    usr.clone(),
                    Some(usr.clone()),
                    MensajesServidor::Invitation {
                        username: (usuario.clone()),
                        roomname: (roomname.clone()) },
                    tx_destino).await;
                }   
            }
            return None;
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

//envia mensajes solo a un usuario previamente especificado (no a todos)
async fn envia_mensajes_secundarios_privados(autor: NombreUsuario, destino: Option<NombreUsuario>, mensaje: MensajesServidor, tx_destino: Sender<EventoChat>) {

    let _ = tx_destino.send(EventoChat{
        autor: autor,
        destino: destino,
        mensaje: mensaje,
    }).await;
    
}
