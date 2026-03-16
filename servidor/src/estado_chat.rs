use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use tokio::sync::broadcast::Sender;
use crate::{cuarto::Cuarto, evento_servidor::EventoChat};
use crate::usuario::Usuario;

use common::nombres::{NombreUsuario, NombreCuarto};


//struct que guarda el diccionario con los usuarios conectados y los cuartos existentes además de guardar cómo enviar mensajes a los usuarios conectados
pub struct EstadoChat {
    pub diccionario_usuarios: RwLock<HashMap<NombreUsuario, Arc<RwLock<Usuario>>>>,
    pub diccionario_cuartos: RwLock<HashMap<NombreCuarto, Arc<RwLock<Cuarto>>>>,
    pub forma_mandar_mensajes: RwLock<HashMap<NombreUsuario, mpsc::Sender<EventoChat>>>,
    pub tx: Sender<EventoChat>,
}


impl EstadoChat {
    //método constructor que inicializa el diccionario de los usuarios conectados y los cuartos existentes
    pub fn new(sender: Sender<EventoChat>) -> EstadoChat {
        Self {
            diccionario_usuarios: RwLock::new(HashMap::new()),
            diccionario_cuartos: RwLock::new(HashMap::new()),
            tx: sender,
            forma_mandar_mensajes: RwLock::new(HashMap::new()),
        }
    }
}


