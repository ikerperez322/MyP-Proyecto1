use serde_json::Result;
use crate::protocolo::{MensajesServidor, MensajesCliente};

/// Serializa un mensaje del servidor a formato JSON.
///
/// # Parámetros
/// - `mensaje`: estructura `MensajesServidor` a convertir.
///
/// # Regresa
/// Una cadena (`String`) en formato JSON que representa el mensaje.
///
/// # Errores
/// Regresa un error si la serialización falla.
pub fn deserializa_json_servidor(mensaje: MensajesServidor) -> Result<String> {
    return serde_json::to_string(&mensaje);
}

/// Serializa un mensaje del servidor a formato JSON.
///
/// # Parámetros
/// - `mensaje`: estructura `MensajesServidor` a convertir.
///
/// # Regresa
/// Una cadena (`String`) en formato JSON que representa el mensaje.
///
/// # Errores
/// Regresa un error si la serialización falla.
pub fn serializa_json_servidor(mensaje: &str) -> Result<MensajesServidor>{
    return serde_json::from_str(&mensaje);
}

/// Serializa un mensaje del cliente a formato JSON.
///
/// # Parámetros
/// - `mensaje`: estructura `MensajesCliente` a convertir.
///
/// # Regresa
/// Una cadena (`String`) en formato JSON que representa el mensaje.
///
/// # Errores
/// Regresa un error si la serialización falla.
pub fn deserializa_json_cliente(mensaje: MensajesCliente) -> Result<String> {
    return serde_json::to_string(&mensaje);
}

/// Serializa un mensaje del cliente a formato JSON.
///
/// # Parámetros
/// - `mensaje`: estructura `MensajesCliente` a convertir.
///
/// # Regresa
/// Una cadena (`String`) en formato JSON que representa el mensaje.
///
/// # Errores
/// Regresa un error si la serialización falla.
pub fn serializa_json_cliente(mensaje: &str) -> Result<MensajesCliente> {
    return serde_json::from_str(&mensaje);
}
