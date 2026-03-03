use tokio::net::TcpStream;
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::env;
use common::{Identificador, deserializar_identificador};


#[tokio::main]
async fn main() -> Result<(), Box<dyn::std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    let direccion_ip = args.get(1).expect("Se debe pasar una dirección IP.");
    let puerto = args.get(2).expect("Se debe pasar un puerto");

    corre_cliente(direccion_ip, puerto).await?;
    
    Ok(())
}


async fn corre_cliente(ip: &str, puerto: &str) -> Result<(), Box<dyn std::error::Error>> {

    let direccion = format!("{}:{}", ip, puerto);
    let stream = TcpStream::connect(direccion).await?;
    println!("Servidor conectado");

    let (reader, writer) = stream.into_split();

    tokio::try_join!(leer_servidor(reader), escribir_servidor(writer))?;
    
    Ok(())
}


async fn leer_servidor(lector: OwnedReadHalf) -> Result<(), Box<dyn std::error::Error>> {

    let mut lector = BufReader::new(lector);
    let mut linea = String::new();

    loop {
        linea.clear();

        let bytes = lector.read_line(&mut linea).await.unwrap();

        if bytes == 0 {
            println!("Servidor desconectado");
            break;
        }

        print!("Servidor dice: {}", linea);
    }
    
    Ok(())
}


async fn escribir_servidor(mut escritor: OwnedWriteHalf) -> Result<(), Box<dyn std::error::Error>> {

    let usuario = Identificador {
        tipo: "IDENTIFY".to_owned(),
        username: "Iker".to_owned(),
    };

    let usuario_string: String = deserializar_identificador(usuario)?;
    
    escritor.write_all(usuario_string.as_bytes()).await.unwrap();
    
    let entrada_estandar = io::stdin();
    let mut input = BufReader::new(entrada_estandar);
    let mut linea = String::new();

    loop {
        linea.clear();

        input.read_line(&mut linea).await.unwrap();

        if linea.trim() == "exit" {
            break;
        }

        escritor.write_all(linea.as_bytes()).await.unwrap();
            
    }

    Ok(())
}


