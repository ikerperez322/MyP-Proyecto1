use common::protocolo::MensajesServidor;
use colored::*;

pub fn representa_info(mensaje_server: MensajesServidor) -> String {

    match mensaje_server {
        MensajesServidor::Response { operation, result, extra } => {

            return match_response(operation, result, extra);
            // let xtra: String = match extra {
            //     Some(e) => e,
            //     None => "".to_string(),
            // };
            // return format!("OPERACIÓN: {0}, RESULTADO: {1}, {2}.", operation, result, xtra);
        }
        MensajesServidor::NewUser { username } => {
            return format!("¡{} SE HA CONECTADO AL CHAT!", username.0).blue().to_string();
        }
        MensajesServidor::NewStatus { username, status } => {
            let estatus: String = match status {
                common::status::Status::ACTIVE => String::from("Activo.").green().to_string(),
                common::status::Status::BUSY => String::from("Ocupado.").yellow().to_string(),
                common::status::Status::AWAY => String::from("No disponible.").dimmed().to_string(),
            };
            
            return format!("{0} HA CAMBIADO SU ESTADO A: {1}", username.0, estatus);
        }
        MensajesServidor::UserList { users } => {
            let mut resultado: String = String::from("USUARIOS DEL CHAT:\n").yellow().bold().to_string();
            for (nombre, tatus) in users.iter() {
                let estatus: String = match tatus {
                    common::status::Status::ACTIVE => String::from("Activo.").green().to_string(),
                    common::status::Status::BUSY => String::from("Ocupado.").yellow().to_string(),
                    common::status::Status::AWAY => String::from("No disponible.").dimmed().to_string(),
                };
                resultado.push_str(&format!("USUARIO: {}, ESTATUS: {}\n", nombre.0, estatus));
            }
            return format!("{}", resultado);
        }
        MensajesServidor::TextFrom { username, text } => {
            return format!("MENSAJE PRIVADO DE: {} ->-> {}", username.0, text).magenta().to_string();
        }
        MensajesServidor::PublicTextFrom { username, text } => {
            return format!("MENSAJE PÚBLICO DE: {} ->-> {}", username.0, text);
        }
        MensajesServidor::Invitation { username, roomname } => {
            return format!("¡{} TE ESTÁ INVITANDO AL CUARTO!: {}", username.0, roomname.0).blue().to_string();
        }
        MensajesServidor::JoinedRoom { roomname , username } => {
            return format!("¡{} SE HA UNIDO AL CUARTO: {}!", username.0, roomname.0).blue().to_string();
        }
        MensajesServidor::RoomUserList { roomname, users } => {
            let mut resultado: String = String::from(format!("USUARIOS DEL CUARTO: {}\n", roomname.0)).yellow().bold().to_string();
            for (nombre, tatus) in users.iter() {
                let estatus: String = match tatus {
                    common::status::Status::ACTIVE => String::from("Activo.").green().to_string(),
                    common::status::Status::BUSY => String::from("Ocupado.").yellow().to_string(),
                    common::status::Status::AWAY => String::from("No disponible.").dimmed().to_string(),
                };
                resultado.push_str(&format!("USUARIO: {}, ESTATUS: {}\n", nombre.0, estatus));
            }
            return format!("{}", resultado);
        }
        MensajesServidor::RoomTextFrom { roomname, username, text } => {
            return format!("MENSAJE AL CUARTO: {} DE: {} ->-> {}", roomname.0, username.0, text).cyan().to_string();
        }
        MensajesServidor::LeftRoom { roomname, username } => {
            return format!("{} HA ABANDONADO EL CUARTO: {}.", username.0, roomname.0).blue().to_string();
        }
        MensajesServidor::Disconnected { username } => {
            return format!("{} SE HA DESCONECTADO DEL CHAT.", username.0).blue().to_string();
        }
    }
}

//función que determina que tipo de response recibe del usuario.
fn match_response(operation: String, result: String, extra: Option<String>) -> String {
    match operation.as_str() {
        "IDENTIFY" => {
            let username: String = match  extra {
                Some(usr) => usr,
                None => "".to_string(),
            };
            if result == "SUCCESS" {
                return format!("TE HAS IDENTIFICADO EXITOSAMENTE CON EL NOMBRE DE: {}.", username).green().to_string();
            }else {
                return format!("EL NOMBRE DE USUARIO: {} YA EXISTE. POR FAVOR, ESCOGE OTRO NOMBRE.", username).red().to_string();
            }        
        }
        "TEXT" => {
            let username: String = match  extra {
                Some(usr) => usr,
                None => "".to_string(),
            };
            return format!("EL USUARIO: {} NO EXISTE. NO SE ENVÍO NINGÚN MENSAJE.", username).yellow().to_string();
        }
        "NEW_ROOM" => {
            let roomname: String = match  extra {
                Some(rmn) => rmn,
                None => "".to_string(),
            };
            if result == "SUCCESS" {
                return format!("CUARTO: {} CREADO EXITOSAMENTE.", roomname).green().to_string();
            } else {
                return format!("EL CUARTO: {} YA EXISTE. POR FAVOR, ELIGE OTRO NOMBRE.", roomname).yellow().to_string();
            }
        }
        "INVITE" => {
            if result == "NO_SUCH_ROOM" {
                let roomname: String = match  extra {
                    Some(rmn) => rmn,
                    None => "".to_string(),
                };
                return format!("EL CUARTO: {} NO EXISTE. NO SE ENVIÓ NINGUNA INVITACIÓN.", roomname).yellow().to_string();
            } else {
                let username: String = match  extra {
                    Some(usr) => usr,
                    None => "".to_string(),
                };
                return format!("EL USUARIO: {} NO EXISTE. NO SE ENVIÓ NINGUNA INVITACIÓN.", username).yellow().to_string();
            }
        }
        "JOIN_ROOM" => {
            let roomname: String = match  extra {
                Some(rmn) => rmn,
                None => "".to_string(),
            };
            if result == "SUCCESS" {
                return format!("TE HAS UNIDO EXITOSAMENTE AL CUARTO: {}", roomname).green().to_string();
            }
                    else if result == "NO_SUCH_ROOM"{
                        return format!("EL CUARTO: {} NO EXISTE. NO TE HAS UNIDO AL CUARTO.", roomname).yellow().to_string();
                    } else {
                        return format!("NO HAS SIDO INVITADO AL CUARTO: {}. NO TE HAS UNIDO AL CUARTO.", roomname).yellow().to_string();
                    }
        }
        "ROOM_USERS" => {
            let roomname: String = match  extra {
                Some(rmn) => rmn,
                None => "".to_string(),
            };
            if result == "NO_SUCH_ROOM" {                        
                return format!("EL CUARTO: {} NO EXISTE. NO SE PUEDE REGRESAR LA LISTA DE USUARIOS.", roomname).yellow().to_string();
            } else {
                return format!("NO HAS SIDO INVITADO AL CUARTO: {}. NO SE PUEDE REGRESAR LA LISTA DE USUARIOS.", roomname).yellow().to_string();
            }
        }
        "ROOM_TEXT" => {
            let roomname: String = match  extra {
                Some(rmn) => rmn,
                None => "".to_string(),
            };
            if result == "NO_SUCH_ROOM" {                        
                return format!("EL CUARTO: {} NO EXISTE. NO SE HA ENVIADO NINGÚN MENSAJE.", roomname).yellow().to_string();
            } else {
                return format!("NO HAS SIDO INVITADO AL CUARTO: {}. NO SE HA ENVIADO NINGÚN MENSAJE.", roomname).yellow().to_string();
            }
        }
        "LEAVE_ROOM" => {
            let roomname: String = match  extra {
                Some(rmn) => rmn,
                None => "".to_string(),
            };
            if result == "NO_SUCH_ROOM" {                        
                return format!("EL CUARTO: {} NO EXISTE. NO HAS ABANDONADO NINGÚN CUARTO.", roomname).yellow().to_string();
            } else {
                return format!("NO HAS SIDO INVITADO AL CUARTO: {}. NO HAS ABANDONADO NINGÚN CUARTO.", roomname).yellow().to_string();
            }
        }
        "INVALID" => {
            if result == "NOT_IDENTIFIED" {
                return format!("NO PUEDES REALIZAR NINGUNA ACCIÓN PORQUE NO TE HAS IDENTIFICADO.").red().to_string();
            } else {
                //aqui habría que desconectar al cliente
                return format!("MENSAJE INVÁLIDO.").red().to_string();
            }
        }
        _ => {
            return "".to_string();
        }
    }
           
}



