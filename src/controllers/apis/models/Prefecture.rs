use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prefecture {
    pub name: String,
    pub code: i32
}