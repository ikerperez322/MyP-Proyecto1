use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Servidor corriendo en 127.0.0.1:8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Nueva conexión desde {}", addr);

        tokio::spawn(async move {
            if let Err(e) = handle_connection(socket).await {
                eprintln!("Error en conexión {}: {}", addr, e);
            }
        });
    }
}

async fn handle_connection(socket: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
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
