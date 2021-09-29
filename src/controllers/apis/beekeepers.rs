use actix_web::{HttpRequest, Responder, HttpResponse};
use crate::models::beekeeper::Beekeeper;
use crate::controllers::apis::models::Beekeeper::Beekeeper as ApiBeekeeper;
use crate::use_cases::apis::beekeepers::get_all_beekeepers_use_case;
use serde::{Serialize, Deserialize};

/**
 * bk_index is used for api
 *  returns all of beekeeper basic information  (beekeeper names, beekeeper location)
 */
pub async fn bk_index(httpReq: HttpRequest) -> impl Responder {
    let requestAdapter = BeekeeperRequestAdapter{};
    let responseAdapter = BeekeeperResponseAdapter{};

    let requestDTO = requestAdapter.adapt(httpReq);
    let result: BeekeeperResponseDTO  = get_all_beekeepers_use_case::get_all(requestDTO);
    let response = responseAdapter.adapt(result);
    HttpResponse::Ok().json(response)
}


//@todo DTO should be put into special space
// temporally shortcut of request, response adaptor
// map request to domain model
// when we will need paging function onto this project, request should contain offset.
mod BeekeeperRequestAdapter {
    use crate::controllers::apis::beekeepers::BeekeeperRequestDTO;

    // now, we do not have any input. So, do nothing and return void response
    pub fn adapt(req: HttpReqest) -> BeekeeperRequestDTO{
        BeekeeperRequestDTO{}
    }
}

pub struct BeekeeperRequestDTO {}

// unmap from logic domain model to response type
mod BeekeeperResponseAdapter{
    pub fn adapt(dto: VBeekeeperResponseDTO) -> Vec<ApiBeekeeper> {
        dto.to_response()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BeekeeperResponseDTO {
    pub(crate) beekeepers: Vec<Beekeeper>
}

impl BeekeeperResponseDTO {
    pub fn to_response(&self) -> Vec<ApiBeekeeper>{
        self.beekeepers.iter().map(|bk|{
            ApiBeekeeper{
                name: bk.name.name.clone(),
                prefecture: bk.prefecture.name.name.clone()
            }
        }).collect()
    }
}