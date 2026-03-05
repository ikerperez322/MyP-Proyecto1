use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
pub struct NombreUsuario(pub String);

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NombreCuarto(pub String);


