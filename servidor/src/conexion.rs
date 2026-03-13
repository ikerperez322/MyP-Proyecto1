use common::nombres::NombreUsuario;
use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::broadcast;
use std::sync::Arc;
use crate::estado_chat::EstadoChat;
use crate::evento_servidor::EventoChat;
use crate::manejador_mensajes;
use common::maneja_json;
use common::protocolo::MensajesCliente;


//método que lee lo que manda el cliente y serializa, deserializa json y envía las respuestas al cliente
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
                    break;
                }

                let msg = match maneja_json::serializa_json_cliente(&linea) {
                    Ok(m) => m,
                    Err(e) => {
                        eprintln!("JSON inválido: {}", e);
                        continue;
                    }
                };        

                
                
                // let respuesta = match manejador_mensajes::procesa_mensaje(msg, estado.clone(), &mut usuario_actual).await {
                //     Some(response) => response,
                //     None
                // };

                if let Some(respuesta) = manejador_mensajes::procesa_mensaje(&msg, estado.clone(), &mut usuario_actual, tx_usuario.clone()).await {

                    let respuesta_json = maneja_json::deserializa_json_servidor(respuesta)?;
                    println!("Servidor: {}", respuesta_json.trim_end());
                    // let respuesta = format!("{}", respuesta_json);
                    writer.write_all(format!("{}\n", respuesta_json).as_bytes()).await?;
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
                
                // writer.write_all(format!("{}\n", msg).as_bytes()).await?;
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
