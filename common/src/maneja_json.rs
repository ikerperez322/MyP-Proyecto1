use serde::de::Error;
// use serde::de::{Error, value::Error};
// use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::protocolo::{MensajesServidor, MensajesCliente};

//pasa el struct a string (para mensajes que recibe el servidor)
pub fn deserializa_json_servidor(mensaje: MensajesServidor) -> Result<String> {
    serde_json::to_string(&mensaje)
}

//pasa el string a struct (mensajes que recibe el servidor)
pub fn serializa_json_servidor(mensaje: &str) -> Result<MensajesServidor>{
    serde_json::from_str(&mensaje)
}

//pasa el struct a string (para mensajes que recibe el cliente)
pub fn deserializa_json_cliente(mensaje: MensajesCliente) -> Result<String> {
    serde_json::to_string(&mensaje)
}

//pasa el string a struct (mensajes que recibe el cliente)
pub fn serializa_json_cliente(mensaje: &str) -> Result<MensajesCliente> {
    serde_json::from_str(&mensaje)
}

