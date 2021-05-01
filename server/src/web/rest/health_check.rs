use actix_web::{HttpResponse, Responder};

/// 健康检查
#[tracing::instrument]
pub async fn health_check() -> impl Responder {
    tracing::info!("健康检查");
    HttpResponse::Ok().json(r#"{"status": "up"}"#)
}
