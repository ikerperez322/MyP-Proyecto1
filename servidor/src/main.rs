use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::sync::RwLock;
use std::collections::HashMap;
use std::env;
use std::sync::{Arc};


use crate::estado_chat::EstadoChat;
// use std::fmt::Error;
pub mod usuario;
pub mod cuarto;
pub mod estado_chat;
pub mod manejador_mensajes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args: Vec<String> = env::args().collect();
    // let puerto = args.get(1).expect("Se debe de pasar un puerto.");
    let puerto = match args.get(1) {
        Some(p) => p,
        None => {
            return Err("Se debe de pasar un número de puerto como argumento.".into());
        }
    };

    let _: u16 = match puerto.parse() {
        Ok(num) => num,
        Err(_) => {
            return Err("El puerto debe ser un número.".into());
        }
    };
    
    correr_servidor(puerto).await?;

    Ok(())
}

//corre el servidor y crea un hilo cada que recibe una nueva conexión
async fn correr_servidor(puerto: &str) -> Result<(), Box<dyn std::error::Error>>{

    let direccion = format!("127.0.0.1:{}", puerto);
    
    let listener = TcpListener::bind(direccion.clone()).await?;
    println!("Servidor corriendo en: {}", direccion);

    //variable con que contiene el diccionario de usuarios y el diccionario de cuartos existentes para pasarselo a cada task de tokio
    let estado = Arc::new(EstadoChat {
        diccionario_usuarios: RwLock::new(HashMap::new()),
        diccionario_cuartos: RwLock::new(HashMap::new()),
    });
    
    loop {

        let (socket, direccion) = listener.accept().await?;
        println!("Nueva conexión desde {}", direccion);
        
        let estado_clonado = estado.clone();

        tokio::spawn(async move {
            if let Err(e) = maneja_conexion(socket, estado_clonado).await {
                eprintln!("Error en conexión {}: {}", direccion, e);
            };
        });
        
        // match listener.accept().await {
        //     Ok((socket, direccion)) => {
        //         println!("Nueva conexión desde {}", direccion);
        //         tokio::spawn(async move {
        //             if let Err(e) = maneja_conexion(socket).await {
        //                 eprintln!("Error en conexión {}: {}", direccion, e);
        //             } 
        //         });
        //     },
        //     Err(e) => {
        //         eprintln!("Error aceptando la conexión: {}", e);
        //         continue;
        //     }   
        // }
    }
}

//Función que va manejando la conexión de cada cliente que se conecta en el servidor
async fn maneja_conexion(socket: TcpStream, estado: Arc<EstadoChat>) -> Result<(), Box<dyn std::error::Error>> {
    let (reader, mut writer) = socket.into_split();

    let mut reader = BufReader::new(reader);
    let mut linea = String::new();

    loop {
        linea.clear();

        let bytes_leidos = reader.read_line(&mut linea).await?;

        if bytes_leidos == 0 {
            println!("Cliente desconectado");
            break;
        }

        println!("Recibido: {}", linea.trim());

        //aqui va a ir una condicion que le diga al servidor que le mandaron para saber como debe responder
        {
            let mut usuarios = estado.diccionario_usuarios.write();
            let mut cuartos = estado.diccionario_cuartos.write();
    
        }
        

        let respuesta = format!("{}\n", linea.trim());
        writer.write_all(respuesta.as_bytes()).await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_prueba() {
        
    }
    
}
