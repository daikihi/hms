use crate::models::honey::{Honey, HoneyName};
use crate::models::beekeeper::{Beekeeper, BeekeeperName};
use crate::models::prefecture::{Prefecture, PrefectureName, PrefectureCode};

pub fn get_all() -> Vec<Honey>{
    vec![
        Honey{
            name: HoneyName{name: "honey_".to_string()},
            beekeeper: Beekeeper{
                name: BeekeeperName{name: "beekeeper".to_string()},
                prefecture: Prefecture{
                    name: PrefectureName{name: "pname".to_string()},
                    code: PrefectureCode{code: 11}
                }
            }
        }
    ]
}