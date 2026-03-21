use std::env;

/// Representa la configuración necesaria para conectar el cliente
/// a un servidor.
///
/// Contiene la dirección IP y el puerto.
pub struct Configuracion {

    /// Dirección IP del servidor.
    pub ip: String,
    
    /// Puerto del servidor.
    pub puerto: u16,
}

impl Configuracion {

    /// Crea una nueva configuración con valores por defecto.
    ///
    /// # Valores por defecto
    /// - IP: `127.0.0.1` (el localhost)
    /// - Puerto: `1234`
    pub fn new() -> Configuracion {
        Self {
            ip: "127.0.0.1".to_string(),
            puerto: 1234,
        }
    }

    /// Intenta leer la configuración desde los argumentos de la línea de comandos.
    ///
    /// Se esperan dos argumentos:
    /// 1. Dirección IP del servidor
    /// 2. Puerto del servidor
    ///
    /// # Errores
    /// Regresa un error si:
    /// - No se proporciona la IP.
    /// - No se proporciona el puerto.
    /// - El puerto no es un número válido (`u16`).
    pub fn lee_argumentos() -> Result<Configuracion, Box<dyn::std::error::Error>> {
        let args: Vec<String> = env::args().collect();

        let direccion_ip = match args.get(1) {
            Some(p) => p,
            None => {
                return Err("Se debe de pasar una dirección IP.".into());
            }
        };

        let puerto = match args.get(2) {
            Some(p) => p,
            None => {
                return Err("Se debe de pasar un puerto.".into());
            }
        };

        let puerto_numerico: u16 = match puerto.parse() {
            Ok(num) => num,
            Err(_) => {
                return Err("El puerto debe de ser un número.".into());
            }
        };

        Ok(Configuracion { ip: (direccion_ip.clone()), puerto: (puerto_numerico) })
    }

}

