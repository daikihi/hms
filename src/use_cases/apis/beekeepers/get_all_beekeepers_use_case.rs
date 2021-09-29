use crate::models::beekeeper::{Beekeeper, BeekeeperName};
use crate::models::prefecture::{Prefecture, PrefectureName, PrefectureCode};

pub fn get_all() -> Vec<Beekeeper> {
    let mielmie = Beekeeper {
        name: BeekeeperName{name: String::from("beekeeper_name")},
        prefecture: Prefecture{
            name: PrefectureName{name: String::from("Kyoto")},
            code: PrefectureCode{code: 10}
        }
    };
    vec![mielmie]
}