use std::{collections::{HashSet, LinkedList}, sync::Arc};
// use tokio::sync::RwLock;
// use crate::usuario::Usuario;
use common::nombres::{NombreCuarto, NombreUsuario};

pub struct Cuarto {
    pub nombre: NombreCuarto,
    pub lista_usuarios: HashSet<NombreUsuario>,
}
