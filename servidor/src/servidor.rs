use tokio::net::TcpListener;
use std::sync::Arc;
use crate::estado_chat::EstadoChat;
use crate::conexion;

//método que corre el servidor y crea un hilo/task cada que recibe una nueva conexión de algún cliente
pub async fn correr_servidor(puerto: &str) -> Result<(), Box<dyn std::error::Error>>{

    let direccion = format!("0.0.0.0:{}", puerto);

    let listener = TcpListener::bind(direccion.clone()).await?;
    println!("Servidor corriendo en: {}", direccion);

    //variable con que contiene el diccionario de usuarios y el diccionario de cuartos existentes para pasarselo a cada task de tokio
    let estado = Arc::new(EstadoChat::new());
    
    loop {

        let (socket, direccion) = listener.accept().await?;
        println!("Nueva conexión desde {}", direccion);
        
        let estado_clonado = estado.clone();

        tokio::spawn(async move {
            if let Err(e) = conexion::maneja_conexion(socket, estado_clonado).await {
                eprintln!("Error en conexión {}: {}", direccion, e);
            };
        });
        
    }
}
