use std::collections::LinkedList;
use common::protocolo::MensajesCliente;
use common::maneja_json::{serializa_json_cliente, deserializa_json_cliente};
use common::status;
use common::nombres::NombreUsuario;

/// Verifica la correcta deserialización y serialización de un mensaje `Identify`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_identify_json() {
    let json = r#"{"type":"IDENTIFY","username":"Kimberly"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::Identify { username } => {
            assert_eq!(username.0, "Kimberly");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena); 
}

/// Verifica la correcta deserialización y serialización de un mensaje `Status`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_status_json() {
    let json = r#"{"type":"STATUS","status":"AWAY"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::Status { status } => {
            assert_eq!(status, &status::Status::AWAY);
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Users`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_users_json() {
    let json = r#"{"type":"USERS"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    assert_eq!(mensaje, MensajesCliente::Users {  });

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Text`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_private_text_json() {
    let json = r#"{"type":"TEXT","username":"Luis","text":"Hola Luis, ¿cómo estás?"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::Text { username, text } => {
            assert_eq!(username.0, "Luis");
            assert_eq!(text, "Hola Luis, ¿cómo estás?");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Public Text`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_public_text_json() {
    let json = r#"{"type":"PUBLIC_TEXT","text":"¡Hola a todos!"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::PublicText { text } => {
            assert_eq!(text, "¡Hola a todos!");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `New Room`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_new_room_json() {
    let json = r#"{"type":"NEW_ROOM","roomname":"Sala 1"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::NewRoom { roomname } => {
            assert_eq!(roomname.0, "Sala 1");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Invite`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_invite_json() {
    let json = r#"{"type":"INVITE","roomname":"Sala 1","usernames":["Luis","Antonio","Fernando"]}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    let mut usuarios: LinkedList<NombreUsuario> = LinkedList::new();
    usuarios.push_back(NombreUsuario("Luis".to_string()));
    usuarios.push_back(NombreUsuario("Antonio".to_string()));
    usuarios.push_back(NombreUsuario("Fernando".to_string()));        
    match &mensaje {
        MensajesCliente::Invite { roomname, usernames } => {
            assert_eq!(roomname.0, "Sala 1");
            assert_eq!(usernames, &usuarios);
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Join Room`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_join_room_json() {
    let json = r#"{"type":"JOIN_ROOM","roomname":"Sala 1"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::JoinRoom { roomname } => {
            assert_eq!(roomname.0, "Sala 1");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Room Users`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_room_users_json() {
    let json = r#"{"type":"ROOM_USERS","roomname":"Sala 1"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::RoomUsers { roomname } => {
            assert_eq!(roomname.0, "Sala 1");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Room Text`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_room_text_json() {
    let json = r#"{"type":"ROOM_TEXT","roomname":"Sala 1","text":"¡Hola sala 1!"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::RoomText { roomname, text } => {
            assert_eq!(roomname.0, "Sala 1");
            assert_eq!(text, "¡Hola sala 1!");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Leave Room`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_leave_room_json() {
    let json = r#"{"type":"LEAVE_ROOM","roomname":"Sala 1"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    match &mensaje {
        MensajesCliente::LeaveRoom { roomname } => {
            assert_eq!(roomname.0, "Sala 1");
        }
        _ => panic!("Cadena json incorrecta"),
    }

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}

/// Verifica la correcta deserialización y serialización de un mensaje `Disconnect`.
///
/// # Descripción
/// - Convierte un JSON a `MensajesCliente`.
/// - Valida que los campos coincidan con los valores esperados.
/// - Serializa nuevamente a JSON y verifica que sea idéntico al original.
#[test]
fn test_parse_disconnect_json() {
    let json = r#"{"type":"DISCONNECT"}"#;
    let mensaje: MensajesCliente = match serializa_json_cliente(json) {
        Ok(msg) => msg,
        Err(e) => panic!("Error serializando json: {}", e),
    };
    assert_eq!(mensaje, MensajesCliente::Disconnect {  });

    let cadena: String = match deserializa_json_cliente(mensaje) {
        Ok(msg) => msg,
        Err(e) => panic!("Error deserializando json: {}", e),
    };
    assert_eq!(json.to_string(), cadena);
}
