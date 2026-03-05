use serde::{Deserialize, Serialize};
pub mod protocolo;
pub mod status;
pub mod nombres;
pub mod maneja_json;

#[derive(Serialize, Deserialize)]
pub struct Identificador {
    pub tipo: String,
    pub username: String,
}

