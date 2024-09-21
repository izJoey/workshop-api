use actix_files::Files;
use actix_web::web::{self, ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::CustomError;
use sqlx::{Executor, PgPool};

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] database_url: String,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Create a connection pool from the database URL
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to create pool");

    // initialize the database if not already initialized
    pool.execute(include_str!("../../db/schema.sql"))
        .await
        .map_err(CustomError::new)?;
    // let pool = actix_web::web::Data::new(pool);

    let film_repository = api_lib::film_repository::PostgresFilmRepository::new(pool);
    let film_repository = actix_web::web::Data::new(film_repository);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/api")
                .app_data(film_repository)
                .configure(api_lib::health::service)
                .configure(
                    api_lib::films::service::<api_lib::film_repository::PostgresFilmRepository>,
                ),
        )
        .service(
            Files::new("/", "static")
                .show_files_listing()
                .index_file("index.html")
                .use_last_modified(true),
        );
    };
    Ok(config.into())
}
