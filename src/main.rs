use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use actix_web::http::header;

use hms_project::controllers::pages::index::index;
use hms_project::controllers::apis::beekeepers::bk_index;
use hms_project::controllers::apis::honeies::honeies_index;
use hms_project::controllers::apis::prefectures::get_all_prefectures;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
     HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                true
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);  
        
        App::new()
        .wrap(cors)
         .route("/", web::get().to(index))
         .route("/api/v1/beekeepers", web::get().to(bk_index))
         .route("/api/v1/honeies", web::get().to(honeies_index))
         .route("/api/v1/prefectures", web::get().to(get_all_prefectures))
      } 
    )
        .bind("127.0.0.1:8080")?
        .run()
        .await
    
}