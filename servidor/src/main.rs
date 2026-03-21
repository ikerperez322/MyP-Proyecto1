pub mod servidor;
pub mod usuario;
pub mod cuarto;
pub mod estado_chat;
pub mod manejador_mensajes;
pub mod conexion;
pub mod configuracion;
pub mod evento_servidor;
use crate::configuracion::Configuracion;

/// Punto de entrada del programa.
///
/// Inicializa la configuración del servidor a partir de los argumentos
/// de la línea de comandos y arranca el servidor de chat.
///
/// # Errores
/// Regresa un error si ocurre algún fallo durante la ejecución
/// del servidor.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let config = match Configuracion::lee_argumentos() {
        Ok(c) => c,
        Err(_) => Configuracion::new(),
    };
    
    servidor::correr_servidor(&config.puerto.to_string()).await?;

    Ok(())
}
