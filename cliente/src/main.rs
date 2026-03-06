// use common::maneja_json;
// use common::protocolo::MensajesServidor;
// use tokio::net::TcpStream;
// use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
// use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
// // use std::env;
// // use common::{protocolo, maneja_json, nombres};
// use common::nombres::{NombreUsuario};

use crate::configuracion::Configuracion;
pub mod configuracion;
pub mod cliente;
pub mod conexion;
pub mod mensajes_cliente;


#[tokio::main]
async fn main() -> Result<(), Box<dyn::std::error::Error>> {

    // let args: Vec<String> = env::args().collect();

    // let direccion_ip = match args.get(1) {
    //     Some(p) => p,
    //     None => {
    //         return Err("Se debe de pasar una dirección IP.".into());
    //     }
    // };

    // let puerto = match args.get(2) {
    //     Some(p) => p,
    //     None => {
    //         return Err("Se debe de pasar un puerto.".into());
    //     }
    // };

    // let _: u16 = match puerto.parse() {
    //     Ok(num) => num,
    //     Err(_) => {
    //         return Err("El puerto debe de ser un número.".into());
    //     }
    // };

    // let c = Configuracion::new();

    let config = match Configuracion::lee_argumentos() {
        Ok(c) => c,
        Err(_) => Configuracion::new(),
    };
    
    cliente::corre_cliente(&config.ip, &config.puerto.to_string()).await?;
    
    Ok(())
}

// //método que se conecta al servidor con la dirección y puerto dados ademas de llamar a los métodos que escriben y leen del servidor
// async fn corre_cliente(ip: &str, puerto: &str) -> Result<(), Box<dyn std::error::Error>> {

//     let direccion = format!("{}:{}", ip, puerto);
//     let stream = TcpStream::connect(direccion).await?;
//     println!("Servidor conectado. Para salir escribe: \"exit\".");
//     println!("Escribe tu username.");
    
//     let (reader, writer) = stream.into_split();

//     tokio::try_join!(leer_servidor(reader), escribir_servidor(writer))?;

//     Ok(())
// }

// //método que lee lo que manda el servidor, por el momento únicamente imprime en la salida estándar lo que va leyendo
// async fn leer_servidor(lector: OwnedReadHalf) -> Result<(), Box<dyn std::error::Error>> {

//     let mut lector = BufReader::new(lector);
//     let mut linea = String::new();

//     loop {
//         linea.clear();

//         let bytes = lector.read_line(&mut linea).await?;
        
//         if bytes == 0 {
//             println!("Servidor desconectado");
//             break;
//         }

//         print!("Servidor dice: {}", linea);
//     }
    
//     Ok(())
// }


// async fn escribir_servidor(mut escritor: OwnedWriteHalf) -> Result<(), Box<dyn std::error::Error>> {

//     // let usuario = Identificador {
//     //     tipo: "IDENTIFY".to_owned(),
//     //     username: "Iker".to_owned(),
//     // };

//     // let usuario_string: String = deserializar_identificador(usuario)?;
    
//     let entrada_estandar = io::stdin();
//     let mut input = BufReader::new(entrada_estandar);
//     let mut linea = String::new();

//     linea.clear();
//     let bytes = input.read_line(&mut linea).await?;

//     if bytes == 0 {
//         return Ok(());
//     }
    
//     let identificador: MensajesServidor = identificarse(linea.trim());
//     let mut json: String = maneja_json::deserializa_json_servidor(identificador)?;
//     json.push('\n');
//     escritor.write_all(json.as_bytes()).await?;

    
//     loop {

//         linea.clear();
        
//         let bytes = input.read_line(&mut linea).await?;

//         if bytes == 0 {
//             break;
//         }
        
//         if linea.trim() == "exit" {
//             break;
//         }

//         escritor.write_all(linea.as_bytes()).await?;
            
//     }

//     Ok(())
// }

// //regresa el struct identify a partir del nombre que escogio el usuario
// fn identificarse(nombre: &str) -> MensajesServidor {
//     let username = NombreUsuario(nombre.to_string());
//     MensajesServidor::Identify { username: (username) }
// }



