use std::collections::HashMap;
// use crate::status::Status;

// enumeración que contiene todas las posibles acciones a realizar por parte del cliente
pub enum AccionCliente {
    Identificarse {
        nombre: String,
    },
    CambiarEstado {
        nuevo_status: String,
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
        usuarios: HashMap<String, String>,
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
}



