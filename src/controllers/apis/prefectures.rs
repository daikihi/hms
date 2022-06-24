use actix_web::{HttpRequest, HttpResponse, Responder};
use crate::controllers::apis::models::Prefecture::Prefecture;
use crate::models::prefecture::Prefecture as ModelPrefecture;
use crate::use_cases::apis::prefectures::get_all_prefectures_use_case::get_all;

pub async fn get_all_prefectures(httpRequest: HttpRequest)  -> impl Responder {
    let _ = get_all_prefecture_request_adapter(httpRequest);
    let result: Vec<ModelPrefecture> = get_all();
    let response: Vec<Prefecture> = get_all_prefecture_response_adapter(result);
    HttpResponse::Ok().json(response)
}

// memo : adapters should be moved to apis.adapters
pub fn get_all_prefecture_request_adapter(httpRequest: HttpRequest) {}

pub fn get_all_prefecture_response_adapter(prefs: Vec<ModelPrefecture>) -> Vec<Prefecture> {
    let resp: Vec<Prefecture> = prefs.iter().map(|p|
        {
            let n:String = p.name.name.to_string();
            let c: i32 = p.code.code;
            Prefecture { name: n, code: c }
        }).collect();
    resp
}

// pub fn get_all_prefecture_response_adapter(prefs: Vec<ModelPrefecture>) -> Vec<Prefecture>{
//     prefs.iter().map(|pref| {
//         Prefecture{
//             name: pref.name.name.clone(),
//             code: pref.code.code
//         }
//     }).collect()
// }

// even if an user kick this api, a system state should be kept as consistent state　(keep idempotent)
// pub fn load_prefecture_master() -> impl Responder {
//     HttpResponse::Ok().body(get_all_prefecture_response_adapter())
// }