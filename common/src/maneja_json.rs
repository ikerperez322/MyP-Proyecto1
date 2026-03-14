// use serde::de::Error;
// use serde::de::{Error, value::Error};
// use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::protocolo::{MensajesServidor, MensajesCliente};

//pasa el struct a string (para mensajes que manda el servidor)
pub fn deserializa_json_servidor(mensaje: MensajesServidor) -> Result<String> {
    return serde_json::to_string(&mensaje);
}

//pasa el string a struct (mensajes que manda el servidor)
pub fn serializa_json_servidor(mensaje: &str) -> Result<MensajesServidor>{
    return serde_json::from_str(&mensaje);
}

//pasa el struct a string (para mensajes que manda el cliente)
pub fn deserializa_json_cliente(mensaje: MensajesCliente) -> Result<String> {
    return serde_json::to_string(&mensaje);
}

//pasa el string a struct (mensajes que manda el cliente)
pub fn serializa_json_cliente(mensaje: &str) -> Result<MensajesCliente> {
    return serde_json::from_str(&mensaje);
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parse_identify_json() {

        let json = r#"{"type":"IDENTIFY","username":"Kimberly"}"#;

        let mensaje: MensajesCliente = match serializa_json_cliente(json) {
            Ok(msg) => msg,
            Err(e) => panic!("Error serializando json: {}", e),
        };
        
        match mensaje {
            MensajesCliente::Identify { username } => {
                assert_eq!(username.0, "Kimberly");
            }
            _ => panic!("Cadena json incorrecta"),
        }
        
    }

    
}




