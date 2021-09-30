use crate::models::prefecture::{Prefecture as ModelPrefecture, PrefectureName, PrefectureCode};

pub fn get_all() -> Vec<ModelPrefecture>{
    vec![
        ModelPrefecture{
            name: PrefectureName{name: "北海道".to_string()},
            code: PrefectureCode{code: 1}
        },
        ModelPrefecture{
            name: PrefectureName{name: "青森県".to_string()},
            code: PrefectureCode{code: 2}
        }
    ]
}