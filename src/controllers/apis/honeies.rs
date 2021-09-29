use actix_web::{HttpRequest, Responder, HttpResponse};
use crate::controllers::apis::models::Honey::Honey;
use crate::use_cases::apis::honeies::get_all_honeies_use_case;
use crate::models::honey::Honey as ModelHoney;

pub async fn honeies_index(httpReq: HttpRequest) -> impl Responder{
    let _ = honey_request();
    let _result: Vec<ModelHoney> =get_all_honeies_use_case::get_all();
    let resp: Vec<Honey> = honey_response(_result);
    HttpResponse::Ok().json(resp)
}

fn honey_request(){}

fn honey_response(honeis: Vec<ModelHoney>) -> Vec<Honey> {
    honeis.iter().map(|honey|{
        Honey{
            name: honey.name.name.clone(),
            beekeeperName: honey.beekeeper.name.name.clone()
        }
    }).collect()
}

