use actix_web::{
    web::{self, ServiceConfig},
    HttpResponse,
};

pub const API_VERSION: &str = "0.0.1";

pub fn service(cfg: &mut ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> HttpResponse {
    HttpResponse::Ok()
        .append_header(("version", API_VERSION))
        .finish()
}

// #[get("/")]
// async fn hello_world() -> &'static str {
//     "Hello World!"
// }
//
// #[get("/version")]
// async fn version(db: actix_web::web::Data<sqlx::PgPool>) -> String {
//     tracing::info!("Getting version");
//     let result: Result<String, sqlx::Error> = sqlx::query_scalar("SELECT version()")
//         .fetch_one(db.get_ref())
//         .await;
//
//     match result {
//         Ok(version) => version,
//         Err(e) => format!("Error: {:?}", e),
//     }
// }
