use crate::mensajes_cliente;
use common::protocolo::MensajesServidor;
use common::maneja_json;

//método para que un cliente se pueda identificar al servidor, regresa el json en forma de string listo para enviar al servidor
pub fn identificarse(nombre: String) -> Result<String, Box<dyn std::error::Error>> {

    let nombre: MensajesServidor = mensajes_cliente::identifica_cliente(&nombre);
    let json_struct: String = maneja_json::deserializa_json_servidor(nombre)?;    
    // let json: String = format!("{}\n", json_struct);
    Ok(json_struct)
}


