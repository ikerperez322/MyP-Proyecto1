use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use common::maneja_json;
use common::protocolo::{MensajesServidor, MensajesCliente};
use common::nombres::NombreUsuario;
use crate::estado_chat::EstadoChat;
use crate::evento_servidor::EventoChat;
use crate::manejador_mensajes;


/// Maneja la conexión de un cliente.
///
/// Lee mensajes del socket y los procesa según el protocolo del chat.
///
/// # Parámetros
/// - `socket`: conexión TCP del cliente.
/// - `estado`: estado compartido del servidor.
///
/// # Errores
/// Retorna un error si falla la lectura o escritura en el socket.
pub async fn maneja_conexion(socket: TcpStream, estado: Arc<EstadoChat>) -> Result<(), Box<dyn std::error::Error>> {

    let (reader, mut writer) = socket.into_split();

    let mut reader = BufReader::new(reader);
    let mut linea = String::new();

    let (tx_usuario, mut rx_usuario) = tokio::sync::mpsc::channel::<EventoChat>(100);
        
    let mut usuario_actual: Option<NombreUsuario> = None;

    let mut rx_broadcast = estado.tx.subscribe();
    
    loop {

        linea.clear();
        
        tokio::select! {
            
            peticion = reader.read_line(&mut linea) => {

                
                println!("Cliente: {}", linea.trim_end());

                if peticion? == 0 {
                    elimina_usuario(estado.clone(), &mut usuario_actual, tx_usuario.clone()).await;
                    break;
                }

                //desconectamos al cliente en caso de que quiera mandar un mensaje de más de 1024 bytes
                if linea.len() > 1024 {
                    let respuesta_json = maneja_json::deserializa_json_servidor(MensajesServidor::Response {
                        operation: ("INVALID".to_string()),
                        result: ("INVALID".to_string()),
                        extra: (None) })?;
                    writer.write_all(format!("{}\n", respuesta_json).as_bytes()).await?;
                    elimina_usuario(estado.clone(), &mut usuario_actual, tx_usuario.clone()).await;
                    break;
                }
                
                //Desconectamos al cliente si manda un json inválido
                let msg = match maneja_json::serializa_json_cliente(&linea) {
                    Ok(m) => m,
                    Err(_) => {
                        let respuesta_json = maneja_json::deserializa_json_servidor(MensajesServidor::Response {
                            operation: ("INVALID".to_string()),
                            result: ("INVALID".to_string()),
                            extra: (None) })?;
                        writer.write_all(format!("{}\n", respuesta_json).as_bytes()).await?;
                        //a prueba esto
                        elimina_usuario(estado.clone(), &mut usuario_actual, tx_usuario.clone()).await;
                        break;
                    }
                };

                if let Some(respuesta) = manejador_mensajes::procesa_mensaje(&msg, estado.clone(), &mut usuario_actual, tx_usuario.clone()).await {

                    let respuesta_json = maneja_json::deserializa_json_servidor(respuesta.clone())?;
                    println!("Servidor: {}", respuesta_json.trim_end());                   
                    writer.write_all(format!("{}\n", respuesta_json).as_bytes()).await?;

                    //checamos si la respuesta del servidor fue que el cliente no se ha identificado para desconectarlo
                    match respuesta {
                        MensajesServidor::Response {operation: _, result, extra: _} => {
                            if result == "NOT_IDENTIFIED" {
                                println!("Cliente no se identificó y quiso realizar otra operación.");
                                break;
                            }
                        },
                        _ => {},
                    }
                }
                                
                if let MensajesCliente::Disconnect {} = msg {
                    println!("Cliente solicitó desconectarse.");
                    break;
                }                
                
            }
            
            Ok(evento) = rx_broadcast.recv() => {

                let mensaje: String = maneja_json::deserializa_json_servidor(evento.mensaje)?;
                
                match &usuario_actual {
                    Some(usuario) => {
                        if *usuario != evento.autor {
                            writer.write_all(
                                format!("{}\n", mensaje).as_bytes()
                            ).await?;
                        }
                    }
                    None => {
                        writer.write_all(
                            format!("{}\n", mensaje).as_bytes()
                        ).await?;
                    }

                }
            }

            Some(evento) = rx_usuario.recv() => {

                let mensaje = maneja_json::deserializa_json_servidor(evento.mensaje)?;

                writer.write_all(
                    format!("{}\n", mensaje).as_bytes()
                ).await?;
            }            
        }        
    }
    Ok(())
}

/// Elimina a un cliente del chat.
///
/// Elimina al usuario del diccionario de usuarios del chat (solo se utiliza en casos extraordinarios, en general se desconecta en: src/manejador_mensajes.rs).
///
/// # Parámetros
/// - `estado`: estado compartido del servidor.
/// - `usuario_actual`: el nombre de usuario del usuario que se va a desconectar.
/// - `tx_destino`: un canal de tokio para enviar eventos.
///
/// # Errores
/// Retorna un error si falla la lectura o escritura en el socket.
async fn elimina_usuario(estado: Arc<EstadoChat>, usuario_actual: &mut Option<NombreUsuario>, tx_destino: Sender<EventoChat>) {
    let mut usuarios = estado.diccionario_usuarios.write().await;
    
    let mut mapa = estado.forma_mandar_mensajes.write().await;
    
    let usuario = match usuario_actual {
        Some(user) => user,
        None => return,
    };
    
    usuarios.remove(usuario);
    mapa.remove(usuario);
    
    let _ = tx_destino.send(EventoChat{
        autor: usuario.clone(),
        destino: None,
        mensaje: MensajesServidor::Disconnected {
            username: (usuario.clone()) },
    }).await;
}

