use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::LinkedList;
use crate::status::Status;
use crate::nombres::{NombreCuarto, NombreUsuario};
// use serde_json::Result;

//Mensajes que recibe el servidor
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MensajesServidor {
    #[serde(rename = "IDENTIFY")]
    Identify {
        username: NombreUsuario,
    },
    #[serde(rename = "NEW_USER")]
    NewUser {
        username: NombreUsuario,
    },
    #[serde(rename = "STATUS")]
    Status {
        status: Status,
    },
    #[serde(rename = "NEW_STATUS")]
    NewStatus {
        username: NombreUsuario,
        status: Status,
    },
    #[serde(rename = "USERS")]
    Users {
        
    },
    #[serde(rename = "USER_LIST")]
    UserList {
        users: HashMap<NombreUsuario, Status>,
    },
    #[serde(rename = "TEXT")]
    Text {
        username: NombreUsuario,
        text: String,
    },
    #[serde(rename = "TEXT_FROM")]
    TextFrom {
        username: NombreUsuario,
        text: String,
    },
    #[serde(rename = "PUBLIC_TEXT")]
    PublicText {
        text: String,
    },
    #[serde(rename = "PUBLIC_TEXT_FROM")]
    PublicTextFrom {
        username: NombreUsuario,
        text: String,
    },
    #[serde(rename = "NEW_ROOM")]
    NewRoom {
        roomname: NombreCuarto,
    },
    #[serde(rename = "INVITE")]
    Invite {
        roomname: String,
        usernames: LinkedList<NombreUsuario>,
    },
    #[serde(rename = "INVITATION")]
    Invitation {
        username: NombreUsuario,
        roomname: NombreCuarto,
    },
    #[serde(rename = "JOIN_ROOM")]
    JoinRoom {
        roomname: NombreCuarto,
    },
    #[serde(rename = "JOINED_ROOM")]
    JoinedRoom {
        roomname: NombreCuarto,
        username: NombreUsuario,
    },
    #[serde(rename = "ROOM_USERS")]
    RoomUsers{
        roomname: NombreCuarto,
    },
    #[serde(rename = "ROOM_USER_LIST")]
    RoomUserList {
        roomname: String,
        users: HashMap<NombreUsuario, Status>,
    },
    #[serde(rename = "ROOM_TEXT")]
    RoomText {
        roomname: NombreCuarto,
        text: String,
    },
    #[serde(rename = "ROOM_TEXT_FROM")]
    RoomTextFrom {
        roomname: NombreCuarto,
        username: NombreUsuario,
        text: String,
    },
    #[serde(rename = "LEAVE_ROOM")]
    LeaveRoom {
        roomname: NombreCuarto,
    },
    #[serde(rename = "LEFT_ROOM")]
    LeftRoom {
        roomname: NombreCuarto,
        username: NombreUsuario,
    },
    #[serde(rename = "DISCONNECT")]
    Disconnect {
        
    },
    #[serde(rename = "DISCONNECTED")]
    Disconnected {
        username: NombreUsuario,
    },
}

//Mensajes que recibe el cliente
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MensajesCliente {
    #[serde(rename = "RESPONSE")]
    Response {
        operation: String,
        result: String,
        extra: String,
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
