use crate::infrastructures::prefectures::files::load_prefectures_from_file;
use crate::models::prefecture::Prefecture;

pub fn load_from_file() -> Vec<Prefecture>{
    let prefectures_str : Vec<Prefecture> = load_prefectures_from_file::load_prefectures_from_file();
    prefectures_str
}

fn get_prefecture_conf() {
    
}

