use std::collections::LinkedList;
use crate::usuario::Usuario;
use common::nombres::NombreCuarto;

pub struct Cuarto {
    pub nombre: NombreCuarto,
    pub lista_usuarios: LinkedList<Usuario>,
}
