use common::{nombres::NombreUsuario, protocolo::MensajesCliente};

//método que recibe un username y regresa el mensaje json a mandar al servidor
pub fn identifica_cliente(nombre: &str) -> MensajesCliente {
    MensajesCliente::Identify {
        username: (NombreUsuario(nombre.to_string()))
    }
}





