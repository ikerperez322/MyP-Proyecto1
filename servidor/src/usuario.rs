use std::collections::HashSet;
use common::nombres::{NombreCuarto, NombreUsuario};
use common::status::Status;

//struct que guarda los atributos de un usuario
pub struct Usuario {
    pub username: NombreUsuario,
    pub status: Status,
    pub cuartos: HashSet<NombreCuarto>,
    pub invitaciones_cuartos: HashSet<NombreCuarto>,
}
