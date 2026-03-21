use tokio::sync::{RwLock, mpsc};
use tokio::sync::broadcast::Sender;
use std::collections::HashMap;
use std::sync::Arc;
use common::nombres::{NombreUsuario, NombreCuarto};
use crate::{cuarto::Cuarto, evento_servidor::EventoChat};
use crate::usuario::Usuario;

/// Representa el estado global del sistema de chat.
///
/// Este struct abstrae:
/// - Los usuarios conectados
/// - Los cuartos existentes
/// - Los canales de comunicación hacia cada usuario
pub struct EstadoChat {
    
    /// Mapa de usuarios conectados en el sistema.
    pub diccionario_usuarios: RwLock<HashMap<NombreUsuario, Arc<RwLock<Usuario>>>>,

    /// Mapa de cuartos existentes en el sistema.
    pub diccionario_cuartos: RwLock<HashMap<NombreCuarto, Arc<RwLock<Cuarto>>>>,

    /// Mapa que asocia a cada usuario con su canal de envío de mensajes.
    pub forma_mandar_mensajes: RwLock<HashMap<NombreUsuario, mpsc::Sender<EventoChat>>>,

    /// Canal principal para enviar eventos dentro del sistema.
    pub tx: Sender<EventoChat>,
}

impl EstadoChat {

    /// Crea una nueva instancia de `EstadoChat`.
    ///
    /// Inicializa las estructuras:
    /// - usuarios conectados
    /// - cuartos existentes
    /// - canales de envío por usuario
    ///
    /// # Parámetros
    /// - `sender`: canal principal para enviar eventos dentro del sistema.
    pub fn new(sender: Sender<EventoChat>) -> EstadoChat {
        Self {
            diccionario_usuarios: RwLock::new(HashMap::new()),
            diccionario_cuartos: RwLock::new(HashMap::new()),
            tx: sender,
            forma_mandar_mensajes: RwLock::new(HashMap::new()),
        }
    }
}

