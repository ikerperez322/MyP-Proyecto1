use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn::std::error::Error>> {

    let args: Vec<String> = env::args().collect();
    let direccion = args.get(1).expect("Se debe pasar una dirección IP.");
    let puerto = args.get(2).expect("Se debe pasar un puerto");

    let direccion = format!("{}:{}", direccion, puerto);

    let mut stream = TcpStream::connect(direccion).await?;
    println!("Servidor conectado");

    stream.write_all(b"Hola al servidor\n").await?;

    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;

    println!("Respuesta: {}", String::from_utf8_lossy(&buffer[..n]));
    
    Ok(())
}
