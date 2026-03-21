use crate::configuracion::Configuracion;
pub mod configuracion;
pub mod cliente;
pub mod conexion;
pub mod acciones_cliente;
pub mod maneja_argumentos;
pub mod vista_terminal;

/// Punto de entrada del programa cliente.
///
/// # Configuración
/// Se intenta obtener:
/// - Dirección IP
/// - Puerto
///
/// desde los argumentos de línea de comandos. En caso de error,
/// se utiliza `Configuracion::new()` con valores por defecto.
///
/// # Errores
/// Regresa un error si:
/// - Falla la conexión con el servidor.
/// - Ocurre un error durante la ejecución del cliente.
#[tokio::main]
async fn main() -> Result<(), Box<dyn::std::error::Error>> {

    let config = match Configuracion::lee_argumentos() {
        Ok(c) => c,
        Err(_) => Configuracion::new(),
    };
    
    cliente::corre_cliente(&config.ip, &config.puerto.to_string()).await?;
    
    Ok(())
}
