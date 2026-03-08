use crate::mensajes_cliente;
use common::protocolo::MensajesCliente;
use common::maneja_json;
use common::acciones_cliente::AccionCliente;

pub fn accion_cliente(accion: AccionCliente) -> Result<String, Box<dyn std::error::Error>>{

    match accion {
        AccionCliente::Identificarse { nombre } => {
            let nombre: MensajesCliente = mensajes_cliente::identifica_cliente(&nombre);
            let json_struct: String = maneja_json::deserializa_json_cliente(nombre)?;    
            Ok(json_struct)
        },
        AccionCliente::CambiarEstado { nuevo_status } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::PedirListaUsuarios {  } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::MandaTextoPrivado { texto, destinatario } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::MandaTextoPublico { texto } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::CreaCuarto { nombre_cuarto } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::InvitaUsuariosCuarto { nombre_cuarto, usuarios } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::UnirseCuarto { nombre_cuarto } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::PedirUsuariosCuarto { nombre_cuarto } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::MandaTextoCuarto { nombre_cuarto, texto } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::AbandonaCuarto { nombre_cuarto } => {
            todo!("Implementar las respuestas del servidor");
        },
        AccionCliente::Desconectarse {  } => {
            todo!("Implementar las respuestas del servidor");
        },
        
    }
    
    
}





// //método para que un cliente se pueda identificar al servidor, regresa el json en forma de string listo para enviar al servidor
// pub fn identificarse(nombre: String) -> Result<String, Box<dyn std::error::Error>> {

//     let nombre: MensajesCliente = mensajes_cliente::identifica_cliente(&nombre);
//     let json_struct: String = maneja_json::deserializa_json_cliente(nombre)?;    
//     // let json: String = format!("{}\n", json_struct);
//     Ok(json_struct)
// }


