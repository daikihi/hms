use crate::models::honey::{Honey, HoneyName};
use crate::models::beekeeper::{Beekeeper, BeekeeperName};
use crate::models::prefecture::{Prefecture, PrefectureName, PrefectureCode};

pub fn get_all() -> Vec<Honey>{
    let honey1 =         Honey{
        name: HoneyName{name: "honey_1".to_string()},
        beekeeper: Beekeeper{
            name: BeekeeperName{name: "beekeeper1".to_string()},
            prefecture: Prefecture{
                name: PrefectureName{name: "pname1".to_string()},
                code: PrefectureCode{code: 11}
            }
        }
    };
    let honey2 =         Honey{
        name: HoneyName{name: "honey_2".to_string()},
        beekeeper: Beekeeper{
            name: BeekeeperName{name: "beekeeper2".to_string()},
            prefecture: Prefecture{
                name: PrefectureName{name: "pname2".to_string()},
                code: PrefectureCode{code: 12}
            }
        }
    };

    vec![honey1, honey2]
}