use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use common::maneja_json;
use crate::acciones_cliente;
use crate::maneja_argumentos;
use crate::vista_terminal;

/// Lee continuamente los mensajes enviados por el servidor.
///
/// Este método:
/// - Recibe un `OwnedReadHalf` del socket TCP.
/// - Lee línea por línea de manera asíncrona.
/// - Intenta deserializar cada mensaje recibido desde JSON.
/// - Muestra en la salida estándar una representación legible del mensaje.
///
/// # Parámetros
/// - `lector`: Mitad de lectura del stream TCP.
///
/// # Regresa
/// - `Ok(())` si la lectura termina correctamente.
/// - `Err(...)` si ocurre un error durante la lectura o procesamiento.
pub async fn leer_servidor(lector: OwnedReadHalf) -> Result<(), Box<dyn std::error::Error>> {

    let mut lector = BufReader::new(lector);
    let mut linea = String::new();

    loop {
        linea.clear();
        
        if lector.read_line(&mut linea).await? == 0 {
            println!("Programa terminado.");
            break;
        }

        match maneja_json::serializa_json_servidor(&linea) {
            Ok(msg) => {
                println!("{}", vista_terminal::representa_info(msg).trim_end());
            },
            Err(e) => {
                println!("Error serializando respuesta del servidor: {}", e);
            }
        };     
    }    
    Ok(())
}

/// Lee la entrada del usuario desde la terminal y envía mensajes al servidor.
///
/// Este método:
/// - Lee líneas desde la entrada estándar (`stdin`).
/// - Convierte cada línea en una acción del cliente.
/// - Serializa la acción a JSON.
/// - Envía el mensaje al servidor.
///
/// # Parámetros
/// - `escritor`: Mitad de escritura del stream TCP.
///
/// # Regresa
/// - `Ok(())` si el ciclo termina correctamente.
/// - `Err(...)` si ocurre un error durante la escritura o lectura.
///
/// # Nota
/// Cada mensaje enviado incluye un salto de línea (`\n`), ya que es el delimitador utilizado para separar jsons.
pub async fn escribir_servidor(mut escritor: OwnedWriteHalf) -> Result<(), Box<dyn std::error::Error>> {
    
    let entrada_estandar = io::stdin();
    let mut input = BufReader::new(entrada_estandar);
    let mut linea = String::new();

    loop {

        linea.clear();
        
        input.read_line(&mut linea).await?;
        
        if linea.trim() == "exit" {
            break;
        }
        
        let mut accion: Option<String> = None;
        
        match acciones_cliente::accion_cliente(maneja_argumentos::determinar_accion(linea.clone())) {
            Ok(json) => {
                accion = Some(json);
            },
            Err(e) => println!("Error: {}", e),
        };
        
        if let Some(valor) = accion {           
            escritor.write_all(format!("{}\n", valor).as_bytes()).await?;
        } else {
            continue;
        }         
    }
    Ok(())
}

