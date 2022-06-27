use crate::infrastructures::beekeepers::files::load_beekeepers_from_file::load_all_beekeepers;
use crate::models::beekeeper::Beekeeper;

pub fn get_all_beekeepers() -> Vec<Beekeeper>{
    load_all_beekeepers()
}