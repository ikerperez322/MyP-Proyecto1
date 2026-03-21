use std::collections::HashMap;
use common::protocolo::MensajesServidor;
use common::maneja_json::{serializa_json_servidor, deserializa_json_servidor};
use common::status;
use common::nombres::NombreUsuario;

/// Verifica la correcta deserialización y serialización de un mensaje `Response`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_response_json() {
    let json = r#"{"type":"RESPONSE","operation":"IDENTIFY","result":"SUCCESS","extra":"Kimberly"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::Response { operation, result, extra } => {
            assert_eq!(operation, &"IDENTIFY".to_string());
            assert_eq!(result, &"SUCCESS".to_string());
            assert_eq!(match extra {
                Some(e) => e,
                None => panic!("Error encontrando extra."),
            }, &"Kimberly");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `New User`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_new_user_json() {
    let json = r#"{"type":"NEW_USER","username":"Luis"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::NewUser { username } => {
            assert_eq!(username.0, "Luis");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `New Status`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_new_status_json() {
    let json = r#"{"type":"NEW_STATUS","username":"Kimberly","status":"AWAY"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::NewStatus { username, status } => {
            assert_eq!(username.0, "Kimberly");
            assert_eq!(status, &status::Status::AWAY);
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `User List`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
#[test]
fn test_parse_user_list_json() {
    let json = r#"{"type":"USER_LIST","users":{"Kimberly":"ACTIVE","Luis":"BUSY","Fernando":"AWAY","Antonio":"ACTIVE"}}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };

    let mut usuarios: HashMap<NombreUsuario, status::Status> = HashMap::new();
    usuarios.insert(NombreUsuario("Kimberly".to_string()), status::Status::ACTIVE);
    usuarios.insert(NombreUsuario("Luis".to_string()), status::Status::BUSY);
    usuarios.insert(NombreUsuario("Fernando".to_string()), status::Status::AWAY);
    usuarios.insert(NombreUsuario("Antonio".to_string()), status::Status::ACTIVE);
        
    match &mensaje {
        MensajesServidor::UserList { users } => {
            assert_eq!(users, &usuarios);
        }
        _ => panic!("Cadena json incorrecta"),
    }
}

/// Verifica la correcta deserialización y serialización de un mensaje `Text From`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_text_from_json() {
    let json = r#"{"type":"TEXT_FROM","username":"Luis","text":"Hola Kim, bien ¿y tú?"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::TextFrom { username, text } => {
            assert_eq!(username.0, "Luis");
            assert_eq!(text, "Hola Kim, bien ¿y tú?");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Public Text From`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_public_text_from_json() {
    let json = r#"{"type":"PUBLIC_TEXT_FROM","username":"Kimberly","text":"¡Hola todos!"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::PublicTextFrom { username, text } => {
            assert_eq!(username.0, "Kimberly");
            assert_eq!(text, "¡Hola todos!");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Invitation`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_invitation_json() {
    let json = r#"{"type":"INVITATION","username":"Kimberly","roomname":"Sala 1"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::Invitation { username, roomname } => {
            assert_eq!(username.0, "Kimberly");
            assert_eq!(roomname.0, "Sala 1");
        }
        _ => panic!("Cadena json incorrecta."),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Joined Room`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_joined_room_json() {
    let json = r#"{"type":"JOINED_ROOM","roomname":"Sala 1","username":"Fernando"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::JoinedRoom { roomname, username } => {
            assert_eq!(roomname.0, "Sala 1");
            assert_eq!(username.0, "Fernando");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Room User List`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
#[test]
fn test_parse_room_user_list_json() {
    let json = r#"{"type":"ROOM_USER_LIST","roomname":"Sala 1","users":{"Kimberly":"ACTIVE","Luis":"AWAY","Antonio":"BUSY","Fernando":"ACTIVE"}}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };

    let mut usuarios: HashMap<NombreUsuario, status::Status> = HashMap::new();
    usuarios.insert(NombreUsuario("Kimberly".to_string()), status::Status::ACTIVE);
    usuarios.insert(NombreUsuario("Luis".to_string()), status::Status::AWAY);
    usuarios.insert(NombreUsuario("Fernando".to_string()), status::Status::ACTIVE);
    usuarios.insert(NombreUsuario("Antonio".to_string()), status::Status::BUSY);
        
    match &mensaje {
        MensajesServidor::RoomUserList { roomname, users } => {
            assert_eq!(roomname.0, "Sala 1");
            assert_eq!(users, &usuarios);
        }
        _ => panic!("Cadena json incorrecta"),
    }
}

/// Verifica la correcta deserialización y serialización de un mensaje `Room Text From`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_room_text_from_json() {
    let json = r#"{"type":"ROOM_TEXT_FROM","roomname":"Sala 1","username":"Kimberly","text":"¡Bienvenidos a mi sala!"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::RoomTextFrom { roomname, username, text } => {
            assert_eq!(roomname.0, "Sala 1");
            assert_eq!(username.0, "Kimberly");
            assert_eq!(text, "¡Bienvenidos a mi sala!");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Left Room`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_left_room_json() {
    let json = r#"{"type":"LEFT_ROOM","roomname":"Sala 1","username":"Fernando"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::LeftRoom { roomname, username } => {
            assert_eq!(roomname.0, "Sala 1");
            assert_eq!(username.0, "Fernando");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Disconnected`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesServidor`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_disconnected_json() {
    let json = r#"{"type":"DISCONNECTED","username":"Luis"}"#;
    let mensaje: MensajesServidor = match serializa_json_servidor(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesServidor::Disconnected { username } => {
            assert_eq!(username.0, "Luis");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_servidor(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}
