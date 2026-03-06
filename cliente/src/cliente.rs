use tokio::net::TcpStream;
use crate::conexion;

//método que se conecta al servidor con la dirección y puerto dados ademas de llamar a los métodos que escriben y leen del servidor
pub async fn corre_cliente(ip: &str, puerto: &str) -> Result<(), Box<dyn std::error::Error>> {

    let direccion = format!("{}:{}", ip, puerto);
    let stream = TcpStream::connect(direccion).await?;
    println!("Servidor conectado. Para salir escribe: \"exit\".");
    println!("Escribe tu username.");
    
    let (reader, writer) = stream.into_split();

    tokio::try_join!(conexion::leer_servidor(reader), conexion::escribir_servidor(writer))?;

    Ok(())
}


