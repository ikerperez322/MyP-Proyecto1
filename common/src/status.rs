use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Status {
    ACTIVE,
    BUSY,
    AWAY,
}
