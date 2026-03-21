use std::collections::HashSet;
use common::nombres::{NombreCuarto, NombreUsuario};

/// Representa un cuarto dentro del chat.
///
/// Un cuarto agrupa a múltiples usuarios que se comunican entre ellos aparte del chat público.
pub struct Cuarto {
    
    /// Nombre único que identifica al cuarto.
    pub nombre: NombreCuarto,
    
    /// Conjunto de usuarios que actualmente pertenecen al cuarto.
    ///
    /// Cada elemento está representado por el nombre de usuario del user.
    pub lista_usuarios: HashSet<NombreUsuario>,
}
