use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args: Vec<String> = env::args().collect();
    let puerto = args.get(1).expect("Se debe de pasar un puerto.");

    correr_servidor(puerto).await?;

    Ok(())
}

async fn correr_servidor(puerto: &str) -> Result<(), Box<dyn std::error::Error>>{

    let direccion = format!("127.0.0.1:{}", puerto);
    
    let listener = TcpListener::bind(direccion.clone()).await?;
    println!("Servidor corriendo en: {}", direccion);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Nueva conexión desde {}", addr);

        tokio::spawn(async move {
            if let Err(e) = maneja_conexion(socket).await {
                eprintln!("Error en conexión {}: {}", addr, e);
            }
        });
    }
}

async fn maneja_conexion(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
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

        let respuesta = format!("Echo: {}\n", linea.trim());
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
