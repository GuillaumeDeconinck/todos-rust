use actix_web::{HttpResponse, Responder};

pub async fn ping_route() -> impl Responder {
    HttpResponse::Ok()
}
