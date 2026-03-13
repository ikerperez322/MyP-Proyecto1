use common::{nombres::NombreUsuario, protocolo::MensajesServidor};
// use std::fmt::

#[derive(Clone, Debug)]
pub struct EventoChat {
    pub autor: NombreUsuario,
    pub destino: Option<NombreUsuario>,
    pub mensaje: MensajesServidor,
}
