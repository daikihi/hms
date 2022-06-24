use crate::models::prefecture::{Prefecture as ModelPrefecture, PrefectureName, PrefectureCode, Prefecture};
use crate::repositories::prefectures::load_prefectures::load_from_file;

// pub fn get_all() -> Vec<ModelPrefecture>{
//     vec![
//         ModelPrefecture{
//             name: PrefectureName{name: "北海道".to_string()},
//             code: PrefectureCode{code: 1}
//         },
//         ModelPrefecture{
//             name: PrefectureName{name: "青森県".to_string()},
//             code: PrefectureCode{code: 2}
//         }
//     ]
// }

pub fn get_all() -> Vec<Prefecture>{
    load_from_file()
}