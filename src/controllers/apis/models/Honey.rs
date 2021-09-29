use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Honey{
    pub name: String,
    pub beekeeperName: String
}

