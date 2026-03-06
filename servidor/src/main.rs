use std::env;
pub mod servidor;
pub mod usuario;
pub mod cuarto;
pub mod estado_chat;
pub mod manejador_mensajes;
pub mod conexion;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let args: Vec<String> = env::args().collect();
    // let puerto = args.get(1).expect("Se debe de pasar un puerto.");
    let puerto = match args.get(1) {
        Some(p) => p,
        None => {
            return Err("Se debe de pasar un número de puerto como argumento.".into());
        }
    };

    let _: u16 = match puerto.parse() {
        Ok(num) => num,
        Err(_) => {
            return Err("El puerto debe ser un número.".into());
        }
    };
    
    servidor::correr_servidor(puerto).await?;

    Ok(())
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_prueba() {
        
    }
    
}
