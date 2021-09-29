use crate::models::beekeeper::Beekeeper;

pub struct Honey{
    pub name: HoneyName,
    pub beekeeper: Beekeeper
}

pub struct HoneyName{
    pub name: String
}