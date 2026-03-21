use tokio::net::TcpListener;
use tokio::sync::broadcast;
use std::sync::Arc;
use crate::estado_chat::EstadoChat;
use crate::conexion;

/// Inicia el servidor de chat y comienza a aceptar conexiones entrantes.
///
/// Escucha en el puerto especificado y, por cada nueva conexión,
/// crea una tarea async independiente para manejar al cliente.
///
/// # Parámetros
/// - `puerto`: puerto en el que el servidor escuchará conexiones.
///
/// # Errores
/// Retorna un error si falla al enlazar el socket o al aceptar conexiones entrantes.
pub async fn correr_servidor(puerto: &str) -> Result<(), Box<dyn std::error::Error>>{

    let direccion = format!("0.0.0.0:{}", puerto);
    let listener = TcpListener::bind(direccion.clone()).await?;
    println!("Servidor corriendo en: {}", direccion);
    let (tx, _) = broadcast::channel(100);
    let estado = Arc::new(EstadoChat::new(tx.clone()));
    
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
