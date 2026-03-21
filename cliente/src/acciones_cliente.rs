use std::collections::LinkedList;
use common::protocolo::MensajesCliente;
use common::maneja_json;
use common::acciones_cliente::AccionCliente;
use common::nombres::{NombreUsuario, NombreCuarto};

/// Convierte una acción del cliente (`AccionCliente`) en un mensaje JSON
/// listo para ser enviado al servidor.
///
/// Esta función actúa como un traductor entre las acciones internas del cliente
/// y la representación en JSON que entiende el servidor. Para cada variante
/// de `AccionCliente`, se construye el mensaje correspondiente de tipo
/// `MensajesCliente` y posteriormente se serializa.
///
/// # Parámetros
/// - `accion`: Acción realizada por el cliente.
///
/// # Regresa
/// - `Ok(String)`: Cadena JSON lista para enviarse al servidor.
/// - `Err(...)`: Si ocurre un error durante la serialización o si la acción es inválida.
///
/// # Errores
/// Regresa un  error si:
/// - La acción es `AccionInvalida`.
/// - Falla la conversión a JSON.
pub fn accion_cliente(accion: AccionCliente) -> Result<String, Box<dyn std::error::Error>>{

    let json_struct: String;
    
    match accion {
        AccionCliente::Identificarse { nombre } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::Identify {
                username: (NombreUsuario(nombre))
            })?;
            return Ok(json_struct);
        },
        AccionCliente::CambiarEstado { nuevo_status } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::Status {
                status: (nuevo_status)
            })?;
            return Ok(json_struct);
        },
        AccionCliente::PedirListaUsuarios {  } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::Users {  })?;
            return Ok(json_struct);
        },
        AccionCliente::MandaTextoPrivado { texto, destinatario } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::Text {
                username: (NombreUsuario(destinatario)),
                text: (texto.to_string()),
            })?;
            return Ok(json_struct);
        },
        AccionCliente::MandaTextoPublico { texto } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::PublicText {
                text: (texto.to_string()),
            })?;
            return Ok(json_struct);
        },
        AccionCliente::CreaCuarto { nombre_cuarto } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::NewRoom {
                roomname: (NombreCuarto(nombre_cuarto)),
            })?;
            return Ok(json_struct);
        },
        AccionCliente::InvitaUsuariosCuarto { nombre_cuarto, usuarios } => {

            let users: LinkedList<NombreUsuario> = usuarios
                .iter()
                .map(|s| NombreUsuario(s.clone()))
                .collect();
            
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::Invite {
                roomname: (NombreCuarto(nombre_cuarto)),
                usernames: (users),
            })?;
            return Ok(json_struct);
        },
        AccionCliente::UnirseCuarto { nombre_cuarto } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::JoinRoom {
                roomname: (NombreCuarto(nombre_cuarto)),
            })?;
            return Ok(json_struct);
        },
        AccionCliente::PedirUsuariosCuarto { nombre_cuarto } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::RoomUsers {
                roomname: (NombreCuarto(nombre_cuarto)),
            })?;
            return Ok(json_struct);
        },
        AccionCliente::MandaTextoCuarto { nombre_cuarto, texto } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::RoomText {
                roomname: (NombreCuarto(nombre_cuarto)),
                text: (texto.to_string()),
            })?;
            return Ok(json_struct);
        },
        AccionCliente::AbandonaCuarto { nombre_cuarto } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::LeaveRoom {
                roomname: (NombreCuarto(nombre_cuarto)),
            })?;
            return Ok(json_struct);
        },
        AccionCliente::Desconectarse {  } => {
            json_struct = maneja_json::deserializa_json_cliente(MensajesCliente::Disconnect {  })?;
            return Ok(json_struct);
        },
        AccionCliente::AccionInvalida {  } => {
            return Err(String::from("Acción inválida").into());
        }
    }
}
