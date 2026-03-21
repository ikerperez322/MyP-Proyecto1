use serde::{Deserialize, Serialize};

/// Representa el identificador único de un usuario.
///
/// Este tipo envuelve un `String` para proporcionar mayor abstracción y seguridad de tipos.
#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq, Eq)]
pub struct NombreUsuario(pub String);

/// Representa el identificador único de un cuarto.
///
/// Este tipo envuelve un `String` para proporcionar mayor abstracción y seguridad de tipos.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NombreCuarto(pub String);

