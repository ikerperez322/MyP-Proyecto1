use std::{collections::LinkedList, sync::Arc};
use tokio::sync::RwLock;
use crate::usuario::Usuario;
use common::nombres::NombreCuarto;

pub struct Cuarto {
    pub nombre: NombreCuarto,
    pub lista_usuarios: LinkedList<Arc<RwLock<Usuario>>>
}
