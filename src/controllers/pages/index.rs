use actix_web::{HttpRequest,  Responder};

pub async fn index(req: HttpRequest) -> impl Responder {
    format!("TEST 1\n")
}
