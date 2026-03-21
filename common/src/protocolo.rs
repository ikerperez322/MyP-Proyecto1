use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::LinkedList;
use crate::status::Status;
use crate::nombres::{NombreCuarto, NombreUsuario};

/// Mensajes que puede enviar el cliente al servidor.
///
/// Este enum define el protocolo de comunicación desde el cliente,
/// indicando las acciones que desea realizar dentro del sistema de chat.
///
/// Cada variante se serializa con un campo `"type"` para identificar
/// el tipo de operación.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum MensajesCliente {
    #[serde(rename = "IDENTIFY")]
    Identify {
        username: NombreUsuario,
    },
    #[serde(rename = "STATUS")]
    Status {
        status: Status,
    },
    #[serde(rename = "USERS")]
    Users {
        
    },
    #[serde(rename = "TEXT")]
    Text {
        username: NombreUsuario,
        text: String,
    },
    #[serde(rename = "PUBLIC_TEXT")]
    PublicText {
        text: String,
    },
    #[serde(rename = "NEW_ROOM")]
    NewRoom {
        roomname: NombreCuarto,
    },
    #[serde(rename = "INVITE")]
    Invite {
        roomname: NombreCuarto,
        usernames: LinkedList<NombreUsuario>,
    },
    #[serde(rename = "JOIN_ROOM")]
    JoinRoom {
        roomname: NombreCuarto,
    },
    #[serde(rename = "ROOM_USERS")]
    RoomUsers{
        roomname: NombreCuarto,
    },
    #[serde(rename = "ROOM_TEXT")]
    RoomText {
        roomname: NombreCuarto,
        text: String,
    },
    #[serde(rename = "LEAVE_ROOM")]
    LeaveRoom {
        roomname: NombreCuarto,
    },
    #[serde(rename = "DISCONNECT")]
    Disconnect {
        
    },
}

/// Mensajes que puede enviar el servidor al cliente.
///
/// Este enum define las respuestas y eventos generados por el servidor,
/// incluyendo notificaciones de estado, mensajes y cambios en cuartos.
///
/// Cada variante se serializa con un campo `"type"` para identificar
/// el tipo de mensaje.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum MensajesServidor {
    #[serde(rename = "RESPONSE")]
    Response {
        operation: String,
        result: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        extra: Option<String>,
    },
    #[serde(rename = "NEW_USER")]
    NewUser {
        username: NombreUsuario,
    },
    #[serde(rename = "NEW_STATUS")]
    NewStatus {
        username: NombreUsuario,
        status: Status,
    },
    #[serde(rename = "USER_LIST")]
    UserList {
        users: HashMap<NombreUsuario, Status>,
    },
    #[serde(rename = "TEXT_FROM")]
    TextFrom {
        username: NombreUsuario,
        text: String,
    },
    #[serde(rename = "PUBLIC_TEXT_FROM")]
    PublicTextFrom {
        username: NombreUsuario,
        text: String,
    },
    #[serde(rename = "INVITATION")]
    Invitation {
        username: NombreUsuario,
        roomname: NombreCuarto,
    },
    #[serde(rename = "JOINED_ROOM")]
    JoinedRoom {
        roomname: NombreCuarto,
        username: NombreUsuario,
    },
    #[serde(rename = "ROOM_USER_LIST")]
    RoomUserList {
        roomname: NombreCuarto,
        users: HashMap<NombreUsuario, Status>,
    },
    #[serde(rename = "ROOM_TEXT_FROM")]
    RoomTextFrom {
        roomname: NombreCuarto,
        username: NombreUsuario,
        text: String,
    },
    #[serde(rename = "LEFT_ROOM")]
    LeftRoom {
        roomname: NombreCuarto,
        username: NombreUsuario,
    },
    #[serde(rename = "DISCONNECTED")]
    Disconnected {
        username: NombreUsuario,  
    },
}

