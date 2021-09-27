use actix_web::{HttpRequest, Responder, HttpResponse};
use crate::models::beekeeper::Beekeeper;
use crate::use_cases::apis::beekeepers::get_all_beekeepers_use_case;
use serde::{Serialize, Deserialize};

/**
 * bk_index is used for api
 *  returns all of beekeeper basic information  (beekeeper names, beekeeper location)
 */
pub async fn bk_index(httpReq: HttpRequest) -> impl Responder {
    let request = BeekeeperRequestModule{req: httpReq};
    let _ = request.to_request();
    let result: Vec<Beekeeper> = get_all_beekeepers_use_case::get_all();
    let resultMod = BeekeeperResponseModule{beekeepers: result};
    HttpResponse::Ok().json(resultMod)
}


// temporally shortcut of request, response adaptor

// map request to domain model
pub struct BeekeeperRequestModule{
    req: HttpRequest
}

impl BeekeeperRequestModule{
    pub fn to_request(&self) {}
}

// unmap from logic domain model to response type
#[derive(Debug, Serialize, Deserialize)]
pub struct BeekeeperResponseModule{
    pub beekeepers: Vec<Beekeeper>
}

impl BeekeeperResponseModule{
    pub fn to_response(&self) -> Vec<Beekeeper>{
        println!("{:?}", self.beekeepers);
        self.beekeepers.to_vec()
    }
}