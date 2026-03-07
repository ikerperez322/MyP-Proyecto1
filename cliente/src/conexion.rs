use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
// use crate::acciones_cliente;
use crate::mensajes_cliente;
use common::maneja_json;
use common::protocolo::MensajesServidor;

//método que lee lo que manda el servidor, por el momento únicamente imprime en la salida estándar lo que va leyendo
pub async fn leer_servidor(lector: OwnedReadHalf) -> Result<(), Box<dyn std::error::Error>> {

    let mut lector = BufReader::new(lector);
    let mut linea = String::new();

    loop {
        linea.clear();
        
        if lector.read_line(&mut linea).await? == 0 {
            println!("Servidor desconectado");
            break;
        }

        println!("Servidor: {}", linea.trim_end())
    }
    
    Ok(())
}

//método que escribe hacia el servidor lo que manda el cliente
pub async fn escribir_servidor(mut escritor: OwnedWriteHalf) -> Result<(), Box<dyn std::error::Error>> {
    
    let entrada_estandar = io::stdin();
    let mut input = BufReader::new(entrada_estandar);
    let mut linea = String::new();

    //identificando al usuario
    input.read_line(&mut linea).await?;
    let nombre: MensajesServidor = mensajes_cliente::identifica_cliente(linea.trim());
    let json: String = format!("{}\n", maneja_json::deserializa_json_servidor(nombre)?);
    escritor.write_all(json.as_bytes()).await?;
    
    loop {

        linea.clear();
        
        input.read_line(&mut linea).await?;
        
        if linea.trim() == "exit" {
            break;
        }

        escritor.write_all(linea.as_bytes()).await?;
    }

    Ok(())
}




