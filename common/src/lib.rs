use serde::{Deserialize, Serialize};
use serde_json::Result;


#[derive(Serialize, Deserialize)]
pub struct Identificador {
    pub tipo: String,
    pub username: String,
}

pub fn deserializar_identificador(usuario: Identificador) -> Result<String> {

    serde_json::to_string(&usuario)
    
}
