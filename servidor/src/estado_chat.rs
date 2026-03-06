use std::collections::HashMap;
use tokio::sync::RwLock;
use crate::cuarto::Cuarto;
use crate::usuario::Usuario;

use common::nombres::{NombreUsuario, NombreCuarto};


//struct que guarda el diccionario con los usuarios conectados y los cuartos existentes
pub struct EstadoChat {
    pub diccionario_usuarios: RwLock<HashMap<NombreUsuario, Usuario>>,
    pub diccionario_cuartos: RwLock<HashMap<NombreCuarto, Cuarto>>,
}


impl EstadoChat {
    //método constructor que inicializa el diccionario de los usuarios conectados y los cuartos existentes
    pub fn new() -> EstadoChat {
        Self {
            diccionario_usuarios: RwLock::new(HashMap::new()),
            diccionario_cuartos: RwLock::new(HashMap::new()),
        }
    }
}


