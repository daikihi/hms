use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prefecture{
    pub name: PrefectureName,
    pub code: PrefectureCode
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrefectureName {
    pub name : String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrefectureCode {
    pub code: i32
}