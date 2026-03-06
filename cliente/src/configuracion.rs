use std::env;

//struct que guarda la dirección ip pasada
pub struct Configuracion {
    pub ip: String,
    pub puerto: u16,
}

impl Configuracion {
    //método constructor que determina la ip como localhost y puerto 8080 por omisión
    pub fn new() -> Configuracion {
        Self {
            ip: "127.0.0.1".to_string(),
            puerto: 8080,
        }
    }

    //intenta leer argumentos de la línea de comandos, en caso de encontrar algún error usa los que están definidos por omisión en el constructor
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

