mod api;
mod model;
mod repository;

use api::trade::get_trades;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use repository::postgresdb::PostgresRepository;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var(
        "DATABASE_URL",
        "postgres://admin:password@localhost:5432/db",
    );

    env_logger::init();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:password@localhost:5432/db")
        .await
        .unwrap();

    HttpServer::new(move || {
        let logger = Logger::default();
        let postgres_repository = PostgresRepository::init(pool.clone());
        let postgres_data = Data::new(postgres_repository);

        App::new()
            .wrap(logger)
            .app_data(postgres_data)
            .service(get_trades)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
