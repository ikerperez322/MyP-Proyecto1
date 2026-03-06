use std::env;

pub struct Configuracion {
    pub puerto: u16,
}

impl Configuracion {

    //método constructor que determina el puerto 1234 por omisión
    pub fn new() -> Configuracion {
        Self {
            puerto: 1234,
        }
    }

    //intenta leer el puerto por la línea de comandos, en caso de encontrar algún error usa el que está definido por omisión en el constructor
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
