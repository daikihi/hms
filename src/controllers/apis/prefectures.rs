use actix_web::{HttpRequest, HttpResponse, Responder};
use crate::controllers::apis::models::Prefecture::Prefecture;
use crate::models::prefecture::Prefecture as ModelPrefecture;
use crate::use_cases::apis::prefectures::get_all_prefectures_use_case::get_all;

pub async fn get_all_prefectures(httpRequest: HttpRequest)  -> impl Responder {
    let _ = get_all_prefecture_request_adapter(httpRequest);
    let result = get_all();
    let response = get_all_prefecture_response_adapter(result);
    HttpResponse::Ok().json(response)
}

pub fn get_all_prefecture_request_adapter(httpRequest: HttpRequest) {}

pub fn get_all_prefecture_response_adapter(prefs: Vec<ModelPrefecture>) -> Vec<Prefecture>{
    prefs.iter().map(|pref| {
        Prefecture{
            name: pref.name.name.clone(),
            code: pref.code.code
        }
    }).collect()
}