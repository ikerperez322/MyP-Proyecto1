use common::nombres::NombreUsuario;
// use std::fmt::

#[derive(Clone, Debug)]
pub struct EventoChat {
    pub autor: NombreUsuario,
    pub mensaje: String,
}
