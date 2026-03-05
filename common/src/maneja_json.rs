// use serde::de::{Error, value::Error};
// use serde::{Deserialize, Serialize};
use serde_json::Result;
use crate::protocolo::{MensajesServidor, MensajesCliente};


pub fn deserializa_json_servidor(mensaje: MensajesServidor) -> Result<String> {
    serde_json::to_string(&mensaje)
}

pub fn serializa_json_servidor(mensaje: String) -> Result<MensajesServidor>{
    serde_json::from_str(&mensaje)
}

pub fn deserializa_json_cliente(mensaje: MensajesCliente) -> Result<String> {
    serde_json::to_string(&mensaje)
}

pub fn serializa_json_cliente(mensaje: String) -> Result<MensajesCliente> {
    serde_json::from_str(&mensaje)
}

