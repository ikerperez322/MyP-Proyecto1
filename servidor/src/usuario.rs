use std::collections::LinkedList;
use std::sync::Arc;
use tokio::sync::RwLock;
use common::nombres::NombreUsuario;
use common::status::Status;
use crate::cuarto::Cuarto;


//struct que guarda los atributos de un usuario
pub struct Usuario {
    pub username: NombreUsuario,
    pub status: Status,
    pub cuartos: LinkedList<Arc<RwLock<Cuarto>>>,
}
