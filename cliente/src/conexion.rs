use common::acciones_cliente::AccionCliente;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
// use crate::acciones_cliente;
use crate::{acciones_cliente, mensajes_cliente};
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

    
    
    // let nombre: MensajesServidor = mensajes_cliente::identifica_cliente(linea.trim());
    //println!("{}", maneja_json::deserializa_json_servidor(nombre)?);
    // let json: String = format!("{}\n", maneja_json::deserializa_json_servidor(nombre)?);

    // let json: String = format!("{}\n", acciones_cliente::identificarse(linea.trim().to_string())?);
    // println!("{}", json);    
    // escritor.write_all(json.as_bytes()).await?;

    determinar_accion(linea.clone());
    
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

//método que recibe la linea que ingreso el usuario (previamente verificada) y determina a que accion del cliente corresponde, regresa un struct indicando que accion quiere realizar el usuario
fn determinar_accion(linea: String) -> AccionCliente {

    let palabras: Vec<&str> = linea.split_whitespace().collect();

    let accion: &str = match palabras.get(0) {
        Some(a) => a,
        None => "none",
    };

    let accion_normalizado = accion.to_lowercase()
        .replace("\n", "")
        .replace("\r", "")
        .replace(" ", "")
        .replace(":", "")
        .replace("_", "");

    let a = accion_normalizado.as_str();
    
    match a {
        "identificarse" => {
            println!("Quieres identificarte.");
        },
        "cambiarestado" => {
            println!("Quieres cambiar de estado.");
        },
        "listausuarios" => {
            println!("Quieres la lista de usuarios.");
        },
        "textoprivado" => {
            println!("Quieres mandar un texto privado.");
        },
        "textopublico" => {
            println!("Quieres mandar un texto público.");
        },
        "creacuarto" => {
            println!("Quieres crear un cuarto.");
        },
        "invitacuarto" => {
            println!("Quieres invitar gente a un cuarto.");
        },
        "unirsecuarto" => {
            println!("Quieres unirte a un cuarto.");
        },
        "usuarioscuarto" => {
            println!("Quieres obtener la lista de usuarios del cuarto.");
        },
        "textocuarto" => {
            println!("Quieres mandar un mensaje al cuarto.");
        },
        "abandonacuarto" => {
            println!("Quieres abandonar el cuarto.");
        },
        "desconectarse" => {
            println!("Quieres desconectarte.");
        },
        _ => println!("No se que quieres hacer."),
    }

    
    // if accion.trim() == "identificarse" {
    //     println!("Te quieres identificar");
    // }

    
    
    println!("Palabras extraídas: {:?}", palabras);

    println!("Primer palabra: {:?}", palabras.get(0));
    
    todo!("Implementar función");
}

//funcion que verifica que lo que ingreso el usuario sea válido
fn verifica_linea(linea: String) -> String {
    todo!("Implementar función");
}




