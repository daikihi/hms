use crate::models::beekeeper::{Beekeeper, BeekeeperName};
use crate::models::prefecture::{Prefecture, PrefectureName, PrefectureCode};
use crate::controllers::apis::beekeepers::{BeekeeperRequestDTO, BeekeeperResponseDTO};

pub fn get_all(reqDTO: BeekeeperRequestDTO) -> BeekeeperResponseDTO {
    let bk1 = Beekeeper {
        name: BeekeeperName{name: String::from("beekeeper_name_1")},
        prefecture: Prefecture{
            name: PrefectureName{name: String::from("Kyoto")},
            code: PrefectureCode{code: 10}
        }
    };
    let bk2 = Beekeeper {
        name: BeekeeperName{name: String::from("beekeeper_name_2")},
        prefecture: Prefecture{
            name: PrefectureName{name: String::from("Kyoto")},
            code: PrefectureCode{code: 10}
        }
    };
    BeekeeperResponseDTO{beekeepers: vec![bk1,bk2]}
}