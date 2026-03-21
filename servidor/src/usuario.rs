use std::collections::HashSet;
use common::nombres::{NombreCuarto, NombreUsuario};
use common::status::Status;

/// Representa a un usuario dentro del chat.
///
/// Contiene la información necesaria para identificar al usuario,
/// su estado actual y su relación con los cuartos disponibles.
pub struct Usuario {
    
    /// Nombre único que identifica al usuario en el sistema.
    pub username: NombreUsuario,
    
    /// Estado actual del usuario (por ejemplo, activo, inactivo, etc.).
    pub status: Status,
    
    /// Conjunto de cuartos a los que el usuario pertenece actualmente.
    pub cuartos: HashSet<NombreCuarto>,
    
    /// Conjunto de cuartos a los que el usuario ha sido invitado, pero que no se ha unido a ellos.
    pub invitaciones_cuartos: HashSet<NombreCuarto>,
}
