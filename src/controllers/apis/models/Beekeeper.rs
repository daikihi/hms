use crate::models::beekeeper::Beekeeper as ModelBeekeeper;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Beekeeper {
    pub name: String,
    pub prefecture: String
}

