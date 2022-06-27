use crate::models::beekeeper::{Beekeeper, BeekeeperName};
use crate::models::prefecture::{Prefecture, PrefectureName, PrefectureCode};
use crate::controllers::apis::beekeepers::{BeekeeperRequestDTO, BeekeeperResponseDTO};
use crate::repositories::beekeepers::get_all_beekeepers;

pub fn get_all(reqDTO: BeekeeperRequestDTO) -> BeekeeperResponseDTO {
    let bks: Vec<Beekeeper> = get_all_beekeepers();
    BeekeeperResponseDTO{beekeepers: bks}
}