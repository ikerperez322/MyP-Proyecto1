use common::{nombres::NombreUsuario, protocolo::MensajesServidor};

//método que recibe un username y regresa el mensaje json a mandar al servidor
pub fn identifica_cliente(nombre: &str) -> MensajesServidor {
    MensajesServidor::Identify {
        username: (NombreUsuario(nombre.to_string()))
    }
}





