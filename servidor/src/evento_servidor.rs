use common::{nombres::NombreUsuario, protocolo::MensajesServidor};

/// Representa un evento dentro del sistema de chat.
///
/// Este struct encapsula la información necesaria para procesar y enviar el mensaje.
#[derive(Clone, Debug)]
pub struct EventoChat {

    /// Usuario que origina el evento o mensaje.
    pub autor: NombreUsuario,

    /// Usuario destino del mensaje.
    ///
    /// - `Some(usuario)` mensaje dirigido a un usuario en especifico.
    /// - `None` mensaje dirigido a todos los usuarios en el chat.
    pub destino: Option<NombreUsuario>,

    /// Contenido del evento, representado como un mensaje del servidor.
    pub mensaje: MensajesServidor,
}
