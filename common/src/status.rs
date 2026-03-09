use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Status {
    ACTIVE,
    BUSY,
    AWAY,
}
