use std::fs;
use crate::models::beekeeper::{Beekeeper, BeekeeperName};
use crate::models::prefecture::{Prefecture, PrefectureCode, PrefectureName};
use crate::repositories::prefectures::load_prefectures::load_from_file;

pub fn load_all_beekeepers() -> Vec<Beekeeper>{
    let file_name = "./data_files/master_data/beekeepers.csv";
    load(file_name)
}

fn load(fileNmae: &str) -> Vec<Beekeeper>{
    let prefectures: Vec<Prefecture> = get_all_prefectures();
    let file_data: String = fs::read_to_string(fileNmae).expect(format!("invalid file name : {} ",fileNmae).as_str());
    file_data.lines().map(|l: &str|
        convert_to_model(l.to_string(), prefectures.clone())
    ).collect()
}

fn get_all_prefectures() -> Vec<Prefecture> {
    load_from_file()
}

fn convert_to_model(bk: String, prefectures: Vec<Prefecture>) -> Beekeeper {
    let v: Vec<&str> = bk.split(',').collect();
    let _: String = v.get(0).unwrap().to_string();
    let bk_name: String = v.get(1).unwrap().to_string();
    let bk_pref: String = v.get(2).unwrap().to_string();

    let pref = prefectures.iter().find(|p| p.name.name == bk_pref).unwrap();

    Beekeeper{
        name: BeekeeperName { name:  bk_name  },
        prefecture: Prefecture { name: pref.clone().name, code: pref.clone().code }
    }
}
