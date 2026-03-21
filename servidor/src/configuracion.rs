use std::env;

/// Representa la configuración del servidor.
///
/// Solo tiene el atributo de puerto para el valor por omisión.
pub struct Configuracion {
    pub puerto: u16,
}

impl Configuracion {

    /// Representa la configuración del servidor.
    pub fn new() -> Configuracion {
        Self {
            puerto: 1234,
        }
    }

    /// Intenta construir la configuración a partir de los argumentos
    /// de la línea de comandos.
    ///
    /// # Errores
    /// Regresa un error si:
    /// - No se proporciona un puerto como argumento.
    /// - El puerto no es un número válido.
    pub fn lee_argumentos() -> Result<Configuracion, Box<dyn::std::error::Error>> {
        
        let args: Vec<String> = env::args().collect();

        let puerto = match args.get(1) {
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

        Ok(Configuracion { puerto: (puerto_numerico) })
        
    }    
}
