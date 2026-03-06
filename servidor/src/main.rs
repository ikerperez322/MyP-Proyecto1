use crate::configuracion::Configuracion;
pub mod servidor;
pub mod usuario;
pub mod cuarto;
pub mod estado_chat;
pub mod manejador_mensajes;
pub mod conexion;
pub mod configuracion;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let config = match Configuracion::lee_argumentos() {
        Ok(c) => c,
        Err(_) => Configuracion::new(),
    };
    
    servidor::correr_servidor(&config.puerto.to_string()).await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_prueba() {
        
    }
    
}
