use tokio::net::TcpStream;
use crate::conexion;

/// Inicia el cliente y establece una conexión con el servidor.
///
/// Este método:
/// - Construye la dirección a partir de la IP y el puerto.
/// - Se conecta al servidor mediante TCP.
/// - Divide el stream en lectura y escritura.
/// - Ejecuta concurrentemente las tareas de lectura y escritura.
///
/// # Parámetros
/// - `ip`: Dirección IP del servidor.
/// - `puerto`: Puerto en el que el servidor está escuchando.
///
/// # Errores
/// Regresa un error si:
/// - No se puede establecer la conexión.
/// - Ocurre un fallo en la lectura o escritura con el servidor.
pub async fn corre_cliente(ip: &str, puerto: &str) -> Result<(), Box<dyn std::error::Error>> {

    let direccion = format!("{}:{}", ip, puerto);
    let stream = TcpStream::connect(direccion).await?;
    println!("Servidor conectado. Para salir escribe: \"exit\".");
    println!("Escribe tu username.");
    
    let (reader, writer) = stream.into_split();

    tokio::try_join!(conexion::leer_servidor(reader), conexion::escribir_servidor(writer))?;

    Ok(())
}

