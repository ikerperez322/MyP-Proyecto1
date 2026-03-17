use common::protocolo::MensajesServidor;

pub fn representa_info(mensaje_server: MensajesServidor) -> String {

    match mensaje_server {
        MensajesServidor::Response { operation, result, extra } => {
            
        }
        MensajesServidor::NewUser { username } => {
            
        }
        MensajesServidor::NewStatus { username, status } => {
            
        }
        MensajesServidor::UserList { users } => {
            
        }
        MensajesServidor::TextFrom { username, text } => {
            
        }
        MensajesServidor::PublicTextFrom { username, text } => {
            
        }
        MensajesServidor::Invitation { username, roomname } => {
            
        }
        MensajesServidor::JoinedRoom { roomname, username } => {
            
        }
        MensajesServidor::RoomUserList { roomname, users } => {
            
        }
        MensajesServidor::RoomTextFrom { roomname, username, text } => {
            
        }
        MensajesServidor::LeftRoom { roomname, username } => {
            
        }
        MensajesServidor::Disconnected { username } => {
            
        }
    }
    todo!("Falta que regrese algo");
}


