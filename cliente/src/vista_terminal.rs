use common::protocolo::MensajesServidor;

pub fn representa_info(mensaje_server: MensajesServidor) -> String {

    match mensaje_server {
        MensajesServidor::Response { operation, result, extra } => {
            let xtra: String = match extra {
                Some(e) => e,
                None => "".to_string(),
            };
            return format!("OPERACIÓN: {0}, RESULTADO: {1}, {2}.", operation, result, xtra);
        }
        MensajesServidor::NewUser { username } => {
            return format!("¡{} SE HA CONECTADO AL CHAT!", username.0);
        }
        MensajesServidor::NewStatus { username, status } => {
            let estatus: String = match status {
                common::status::Status::ACTIVE => String::from("Activo."),
                common::status::Status::BUSY => String::from("Ocupado."),
                common::status::Status::AWAY => String::from("No disponible."),
            };
            
            return format!("{0} HA CAMBIADO SU ESTADO A: {1}", username.0, estatus);
        }
        MensajesServidor::UserList { users } => {
            let mut resultado: String = String::from("USUARIOS DEL CHAT:\n");
            for (nombre, tatus) in users.iter() {
                let estatus: String = match tatus {
                common::status::Status::ACTIVE => String::from("Activo."),
                common::status::Status::BUSY => String::from("Ocupado."),
                common::status::Status::AWAY => String::from("No disponible."),
                };
                resultado.push_str(&format!("USUARIO: {}, ESTATUS: {}\n", nombre.0, estatus));
            }
            return format!("{}", resultado);
        }
        MensajesServidor::TextFrom { username, text } => {
            return format!("MENSAJE PRIVADO DE: {} ->-> {}", username.0, text);
        }
        MensajesServidor::PublicTextFrom { username, text } => {
            return format!("MENSAJE PÚBLICO DE: {} ->-> {}", username.0, text);
        }
        MensajesServidor::Invitation { username, roomname } => {
            return format!("¡{} TE ESTÁ INVITANDO AL CUARTO!: {}", username.0, roomname.0);
        }
        MensajesServidor::JoinedRoom { roomname: _, username } => {
            return format!("¡{} SE HA UNIDO AL CUARTO!", username.0);
        }
        MensajesServidor::RoomUserList { roomname, users } => {
            let mut resultado: String = String::from(format!("USUARIOS DEL CUARTO: {}\n", roomname.0));
            for (nombre, tatus) in users.iter() {
                let estatus: String = match tatus {
                    common::status::Status::ACTIVE => String::from("Activo."),
                    common::status::Status::BUSY => String::from("Ocupado."),
                    common::status::Status::AWAY => String::from("No disponible."),
                };
                resultado.push_str(&format!("USUARIO: {}, ESTATUS: {}\n", nombre.0, estatus));
            }
            return format!("{}", resultado);
        }
        MensajesServidor::RoomTextFrom { roomname, username, text } => {
            return format!("MENSAJE AL CUARTO: {} DE: {} ->-> {}", roomname.0, username.0, text);
        }
        MensajesServidor::LeftRoom { roomname, username } => {
            return format!("{} HA ABANDONADO EL CUARTO: {}.", username.0, roomname.0);
        }
        MensajesServidor::Disconnected { username } => {
            return format!("{} SE HA DESCONECTADO DEL CHAT.", username.0);
        }
    }
}


