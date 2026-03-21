use serde::{Serialize, Deserialize};

/// Representa el estado actual de un usuario dentro del sistema de chat.
///
/// Este estado indica la disponibilidad del usuario para interactuar
/// con otros dentro de la aplicación (definido según el protocolo).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    ACTIVE,
    BUSY,
    AWAY,
}
