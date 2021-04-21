use crate::models::prefecture::Prefecture;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Beekeeper {
    pub name: BeekeeperName,
    pub prefecture: Prefecture
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BeekeeperName {
    pub name: String
}

