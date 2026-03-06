use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::sync::Arc;
use crate::estado_chat::EstadoChat;
use crate::manejador_mensajes;
use common::maneja_json;

//método que lee lo que manda el cliente y serializa, deserializa json y envía las respuestas al cliente
pub async fn maneja_conexion(socket: TcpStream, estado: Arc<EstadoChat>) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = socket.into_split();

    let mut reader = BufReader::new(reader);
    let mut linea = String::new();

    
    loop {
        linea.clear();

        if reader.read_line(&mut linea).await? == 0 {
            break;
        }
        
        println!("Cliente: {}", linea.trim_end());
        
        let msg = match maneja_json::serializa_json_servidor(&linea) {
            Ok(m) => m,
            Err(e) => {
                eprintln!("JSON inválido: {}", e);
                continue;
            }
        };
    
        let respuesta = manejador_mensajes::procesa_mensaje(msg, estado.clone()).await;
        let respuesta_json = maneja_json::deserializa_json_cliente(respuesta)?;
        
        // let respuesta = format!("{}\n", linea.trim());
        println!("Servidor: {}", respuesta_json.trim_end());
        let respuesta = format!("{}\n", respuesta_json);
        writer.write_all(respuesta.as_bytes()).await?;
    }

    Ok(())
}



