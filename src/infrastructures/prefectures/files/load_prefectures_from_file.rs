use std::fs;
use crate::models::prefecture::{Prefecture, PrefectureCode, PrefectureName};

pub fn load_prefectures_from_file() -> Vec<Prefecture> {
    let file_name: &str = "./data_files/master_data/prefectures.csv";
    let pref_str: Vec<Prefecture> = load(file_name);
    pref_str
}

fn load(file_name: &str) -> Vec<Prefecture> {
    let result: String = fs::read_to_string(file_name).expect("invalid file");
    let res : Vec<Prefecture> = result.lines().map(|l|
        convert_to_model(l.to_string())
    ).collect();
    res
}

fn convert_to_model(pref: String) -> Prefecture {
    let v: Vec<&str> = pref.split(",").collect();
    let codeMode: i32 =  v.get(0).unwrap().to_string().parse().unwrap();
    let name : String = v.get(1).unwrap().to_string();
    Prefecture{
        name: PrefectureName { name: name },
        code: PrefectureCode { code:  codeMode}
    }
}