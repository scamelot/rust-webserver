use actix_web::{web, HttpRequest, HttpResponse};
use serde::Serialize;
use mongodb::Client;

#[derive(Serialize)]
pub struct ApiResponse {
    pub message: String,
}

pub async fn hello(req: HttpRequest, client: web::Data<Client>) -> HttpResponse {
    // Now we use the client parameter instead of req.app_data::<Client>()
    let mongodb_client = client.get_ref();
    // You can now use the mongodb_client for database operations.

    let response = ApiResponse {
        message: "Hello, from the API!".to_string(),
    };

    HttpResponse::Ok().json(response)
}
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/hello", web::get().to(hello));
}
