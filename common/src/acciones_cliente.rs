use std::collections::LinkedList;
use crate::{status::Status};
// use crate::status::Status;

// enumeración que contiene todas las posibles acciones a realizar por parte del cliente
// #[derive(PartialEq, Debug)]
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



