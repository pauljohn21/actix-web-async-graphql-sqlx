use actix_web::{Responder, HttpResponse};

/// 健康检查
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("{'status': 'up'}")
}