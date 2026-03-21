use std::collections::LinkedList;
use crate::status::Status;

/// Representa las acciones que puede realizar un cliente dentro del sistema.
///
/// Este enum modela las operaciones ya interpretadas a partir de los mensajes
/// recibidos (por ejemplo, después de deserializar JSON), facilitando la lógica
/// interna del servidor.
///
/// A diferencia de `MensajesCliente`, este enum se utiliza como una
/// representación más directa de las acciones a ejecutar (cada accion corresponde uno a uno con un struct de Mensajes cliente).
pub enum AccionCliente {
    Identificarse {
        nombre: String,
    },
    CambiarEstado {
        nuevo_status: Status,
    },
    PedirListaUsuarios {
        
    },
    MandaTextoPrivado {
        texto: String,
        destinatario: String,
    },
    MandaTextoPublico {
        texto: String,
    },
    CreaCuarto {
        nombre_cuarto: String,
    },
    InvitaUsuariosCuarto {
        nombre_cuarto: String,
        usuarios: LinkedList<String>
    },
    UnirseCuarto {
        nombre_cuarto: String,
    },
    PedirUsuariosCuarto {
        nombre_cuarto: String,
    },
    MandaTextoCuarto {
        nombre_cuarto: String,
        texto: String,
    },
    AbandonaCuarto {
        nombre_cuarto: String,
    },
    Desconectarse {
        
    },
    AccionInvalida {
        
    }
}

