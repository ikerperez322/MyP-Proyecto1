// use std::io::Write;
use std::sync::Arc;
use std::collections::{HashMap, HashSet};
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
             
            }
            //usuario no existe, el usuario se identifica con éxito
            let mut mensajes_usuarios = estado.forma_mandar_mensajes.write().await;
            mensajes_usuarios.insert(username.clone(), sender);
                
            usuarios.insert(username.clone(), Arc::new(RwLock::new(Usuario{
                username: username.clone(),
                status: Status::ACTIVE,
                cuartos: HashSet::new(),
                invitaciones_cuartos: HashSet::new(),
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
               
            return None;
        }
        MensajesCliente::Users {  } => {

            if usuario_actual.is_none() {
                return Some(MensajesServidor::Response{
                    operation: ("INVALID".to_string()),
                    result: ("NOT_IDENTIFIED".to_string()),
                    extra: (None) });
            }

            let usuarios_vector: Vec<(NombreUsuario, Arc<RwLock<Usuario>>)> = {
                let usuarios = estado.diccionario_usuarios.read().await;
                usuarios.iter()
                    .map(|(k, v)| (k.clone(), Arc::clone(v)))
                    .collect()
            };

            let mut lista_usuarios: HashMap<NombreUsuario, Status> = HashMap::new();

            for (nombre, usr_arc) in usuarios_vector {
                let user_lock = usr_arc.read().await;
                lista_usuarios.insert(nombre, user_lock.status.clone());
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

            let instancia_usuario = {
                let usuarios = estado.diccionario_usuarios.read().await;
                usuarios.get(&usuario).cloned()
            };
            
            let instancia_usuario = match instancia_usuario {
                Some(u) => u,
                None => return None,
            };

            {
                let mut cuartos = estado.diccionario_cuartos.write().await;
                
                if cuartos.contains_key(&roomname) {
                    return Some(MensajesServidor::Response {
                        operation: "NEW_ROOM".to_string(),
                        result: "ROOM_ALREADY_EXISTS".to_string(),
                        extra: Some(roomname.0.clone())
                    });
                }

                //agregamos al usuario a la lista de usuarios del cuarto
                let mut lista = HashSet::new();
                lista.insert(usuario.clone());
                
                let cuarto = Arc::new(RwLock::new(Cuarto {
                    nombre: roomname.clone(),
                    lista_usuarios: lista,
                }));
                
                cuartos.insert(roomname.clone(), Arc::clone(&cuarto));
            }
                
            //agregamos el cuarto a la lista de cuartos del usuario
            {
                let mut usuario_lock = instancia_usuario.write().await;
                // usuario_lock.cuartos.push_back(Arc::clone(&cuarto_nuevo));
                usuario_lock.cuartos.insert(roomname.clone());
            }
      
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

            let instancia_cuarto = match cuartos.get(&roomname) {
                Some(crto) => Arc::clone(crto),
                None => return Some(MensajesServidor::Response {
                    operation: ("INVITE".to_string()),
                    result: ("NO_SUCH_ROOM".to_string()),
                    extra: (Some(roomname.0.clone())) }),
            };
            
            //uno o más usuarios no existen
            for usr in usernames.iter() {
                if !usuarios.contains_key(usr) {
                    return Some(MensajesServidor::Response {
                        operation: ("INVITE".to_string()),
                        result: ("NO_SUCH_USER".to_string()),
                        extra: (Some(usr.0.clone())) });
                }

                let instancia_usuario = match usuarios.get(&usr) {
                    Some(usr) => Arc::clone(usr),
                    None => return None,
                };

                //verificamos si el usuario ya está en el cuarto para que no se le envíe la invitación
                {
                    let cuarto_lock = instancia_cuarto.read().await;
                    if cuarto_lock.lista_usuarios.contains(usr) {
                        continue;
                    }
                }

                //verificamos si el usuario ya recibió una invitación previamente para que no reciba una invitación a un mismo cuarto más de una vez
                {
                    let mut usuario_lock = instancia_usuario.write().await;
                    if usuario_lock.invitaciones_cuartos.contains(&roomname) {
                        continue;
                    }
                    usuario_lock.invitaciones_cuartos.insert(roomname.clone());
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

            let cuartos = estado.diccionario_cuartos.read().await;

            let instancia_cuarto = match cuartos.get(&roomname) {
                Some(crto) => Arc::clone(crto),
                //de una vez checamos que el cuarto exista
                None => return Some(MensajesServidor::Response {
                    operation: ("JOIN_ROOM".to_string()),
                    result: ("NO_SUCH_ROOM".to_string()),
                    extra: (Some(roomname.0.clone())) }),
            };
            
            let usuarios = estado.diccionario_usuarios.read().await;

            let instancia_usuario = match usuarios.get(&usuario) {
                Some(usr) => Arc::clone(usr),
                None => return None,
            };

            //checamos que el usuario haya sido invitado al cuarto
            {
                let mut usuario_lock = instancia_usuario.write().await;
                if !usuario_lock.invitaciones_cuartos.contains(&roomname) {
                    return Some(MensajesServidor::Response {
                        operation: ("JOIN_ROOM".to_string()),
                        result: ("NOT_INVITED".to_string()),
                        extra: (Some(roomname.0.clone())) });
                }
                usuario_lock.invitaciones_cuartos.remove(&roomname);
                // usuario_lock.cuartos.push_back(instancia_cuarto.clone());
                usuario_lock.cuartos.insert(roomname.clone());
            }

            //avisamos a los demás usuarios del cuarto que se ha unido un nuevo usuario al cuarto
            {
                let mut cuarto_lock = instancia_cuarto.write().await;
                for usr in cuarto_lock.lista_usuarios.iter() {
                    let tx_destino = {
                        let mapa = estado.forma_mandar_mensajes.read().await;
                        mapa.get(usr).cloned()
                    };
                    if let Some(tx_destino) = tx_destino {
                        envia_mensajes_secundarios_privados(
                            usuario.clone(),
                            Some(usr.clone()),
                            MensajesServidor::JoinedRoom {
                                roomname: (roomname.clone()),
                                username: (usuario.clone()) },
                            tx_destino).await;
                    }
                }
                //agregamos al usuario a la lista de usuarios del cuarto
                cuarto_lock.lista_usuarios.insert(usuario.clone());
            }
            
            return Some(MensajesServidor::Response {
                operation: ("JOIN_ROOM".to_string()),
                result: ("SUCCESS".to_string()),
                extra: (Some(roomname.0.clone())) });
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

            let instancia_cuarto = {
                let cuartos = estado.diccionario_cuartos.read().await;
                
                match cuartos.get(&roomname) {
                    Some(crto) => Arc::clone(crto),
                    None => {
                        return Some(MensajesServidor::Response {
                            operation: "ROOM_USERS".to_string(),
                            result: "NO_SUCH_ROOM".to_string(),
                            extra: Some(roomname.0.clone())
                        });
                    }
                }
            };
            
            let mut lista_usuarios: HashMap<NombreUsuario, Status> = HashMap::new();
            
            //checamos que el usuario pertenezca al cuarto
            let usuarios_en_cuarto = {
                let cuarto_lock = instancia_cuarto.read().await;
                
                if !cuarto_lock.lista_usuarios.contains(&usuario) {
                    return Some(MensajesServidor::Response {
                        operation: "ROOM_USERS".to_string(),
                        result: "NOT_JOINED".to_string(),
                        extra: Some(roomname.0.clone())
                    });
                }
                
                cuarto_lock.lista_usuarios.clone()
            };

            for user in usuarios_en_cuarto {
                
                let instancia_usuario = {
                    let usuarios = estado.diccionario_usuarios.read().await;
                    usuarios.get(&user).cloned()
                };
                
                let instancia_usuario = match instancia_usuario {
                    Some(u) => u,
                    None => continue,
                };
                
                let usuario_lock = instancia_usuario.read().await;
                
                lista_usuarios.insert(user.clone(), usuario_lock.status.clone());
            }
            
            return Some(MensajesServidor::RoomUserList {
                roomname: (roomname.clone()),
                users: (lista_usuarios) });
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

            let cuartos = estado.diccionario_cuartos.read().await;

            let instancia_cuarto = match cuartos.get(&roomname) {
                Some(crto) => Arc::clone(crto),
                //de una vez checamos que el cuarto exista
                None => return Some(MensajesServidor::Response {
                    operation: ("ROOM_TEXT".to_string()),
                    result: ("NO_SUCH_ROOM".to_string()),
                    extra: (Some(roomname.0.clone())) }),
            };

            //checamos que el usuario pertenezca al cuarto
            {
                let cuarto_lock = instancia_cuarto.read().await;
                if !cuarto_lock.lista_usuarios.contains(&usuario) {
                    return Some(MensajesServidor::Response {
                        operation: ("ROOM_TEXT".to_string()),
                        result: ("NOT_JOINED".to_string()),
                        extra: (Some(roomname.0.clone())) });
                }

                for usr in cuarto_lock.lista_usuarios.iter() {
                    if usr == usuario {
                        continue;
                    }
                    let tx_destino = {
                        let mapa = estado.forma_mandar_mensajes.read().await;
                        mapa.get(usr).cloned()
                    };
                    if let Some(tx_destino) = tx_destino {
                        envia_mensajes_secundarios_privados(
                            usuario.clone(),
                            Some(usr.clone()),
                            MensajesServidor::RoomTextFrom {
                                roomname: (roomname.clone()),
                                username: (usuario.clone()),
                                text: (text.to_string()) },
                            tx_destino).await;
                    }
                }
                
            }
            return None;
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

            let instancia_cuarto = {
                let cuartos = estado.diccionario_cuartos.read().await;
                
                match cuartos.get(&roomname) {
                    Some(crto) => Arc::clone(crto),
                    None => {
                        return Some(MensajesServidor::Response {
                            operation: "LEAVE_ROOM".to_string(),
                            result: "NO_SUCH_ROOM".to_string(),
                            extra: Some(roomname.0.clone())
                        });
                    }
                }
            };

            let usuarios_restantes = {
                let mut cuarto_lock = instancia_cuarto.write().await;
                
                if !cuarto_lock.lista_usuarios.contains(&usuario) {
                    return Some(MensajesServidor::Response {
                        operation: "LEAVE_ROOM".to_string(),
                        result: "NOT_JOINED".to_string(),
                        extra: Some(roomname.0.clone())
                    });
                }
                
                cuarto_lock.lista_usuarios.remove(&usuario);
                
                cuarto_lock.lista_usuarios.clone()
            };
            
            if usuarios_restantes.is_empty() {
                let mut cuartos = estado.diccionario_cuartos.write().await;
                cuartos.remove(&roomname);
            }

            let instancia_usuario = {
                let usuarios = estado.diccionario_usuarios.read().await;
                usuarios.get(&usuario).cloned()
            };
            
            let instancia_usuario = match instancia_usuario {
                Some(u) => u,
                None => return None,
            };
            
            //Quitamos el cuarto de la lista de usuarios del usuario
            {
                let mut usuario_lock = instancia_usuario.write().await;
                usuario_lock.cuartos.remove(roomname);
            }
            
            
            for usr in usuarios_restantes {
                
                let tx_destino = {
                    let mapa = estado.forma_mandar_mensajes.read().await;
                    mapa.get(&usr).cloned()
                };
                
                if let Some(tx_destino) = tx_destino {
                    envia_mensajes_secundarios_privados(
                        usuario.clone(),
                        Some(usr.clone()),
                        MensajesServidor::LeftRoom {
                            roomname: roomname.clone(),
                            username: usuario.clone()
                        },
                        tx_destino
                    ).await;
                }
            }
            
            return None;
        }
        MensajesCliente::Disconnect {  } => {
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

            let instancia_usuario = {
                let usuarios = estado.diccionario_usuarios.read().await;
                usuarios.get(&usuario).cloned()
            };

            let instancia_usuario = match instancia_usuario {
                Some(u) => u,
                None => return None,
            };
            
            let cuartos_usuario = {
                let usuario_lock = instancia_usuario.read().await;
                usuario_lock.cuartos.clone()
            };
                   
            //eliminamos todos los cuartos del usuario eliminado
            {
                let mut usuario_lock = instancia_usuario.write().await;
                usuario_lock.cuartos.clear();
                usuario_lock.invitaciones_cuartos.clear();
            }
            
            //eliminamos al usuario de la lista de usuarios de los cuartos donde haya estado
            {
                for crto in cuartos_usuario.iter() {
                    let instancia_cuarto = {
                        let cuartos = estado.diccionario_cuartos.read().await;
                        cuartos.get(crto).cloned()
                    };
                    
                    let instancia_cuarto = match instancia_cuarto {
                        Some(c) => c,
                        None => continue,
                    };

                    let (usuarios_restantes, eliminar_cuarto) = {
                        let mut cuarto_lock = instancia_cuarto.write().await;
                        
                        cuarto_lock.lista_usuarios.remove(&usuario);
                        
                        let es_vacio = cuarto_lock.lista_usuarios.is_empty();
                        
                        (cuarto_lock.lista_usuarios.clone(), es_vacio)
                    };

                    if eliminar_cuarto {
                        let mut cuartos = estado.diccionario_cuartos.write().await;
                        cuartos.remove(crto);
                    }
                    //mandamos el mensaje left_room a los demás miembros del cuarto
                    for usr in usuarios_restantes {
                        let tx_destino = {
                            let mapa = estado.forma_mandar_mensajes.read().await;
                            mapa.get(&usr).cloned()
                        };
                        
                        if let Some(tx_destino) = tx_destino {
                            envia_mensajes_secundarios_privados(
                                usuario.clone(),
                                Some(usr.clone()),
                                MensajesServidor::LeftRoom {
                                    roomname: crto.clone(),
                                    username: usuario.clone()
                                },
                                tx_destino
                            ).await;
                        }
                    }
                }

            }
            
            //eliminamos al usuario de la lista de usuarios del chat
            {
                let mut usuarios = estado.diccionario_usuarios.write().await;
                usuarios.remove(usuario);
            }

            //eliminamos al usuario de la forma de mandar mensajes
            {
                let mut mapa = estado.forma_mandar_mensajes.write().await;
                mapa.remove(usuario);
            }

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
