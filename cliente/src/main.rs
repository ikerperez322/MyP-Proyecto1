use crate::configuracion::Configuracion;
pub mod configuracion;
pub mod cliente;
pub mod conexion;
pub mod mensajes_cliente;
pub mod acciones_cliente;
pub mod maneja_argumentos;

#[tokio::main]
async fn main() -> Result<(), Box<dyn::std::error::Error>> {

    let config = match Configuracion::lee_argumentos() {
        Ok(c) => c,
        Err(_) => Configuracion::new(),
    };
    
    cliente::corre_cliente(&config.ip, &config.puerto.to_string()).await?;
    
    Ok(())
}



