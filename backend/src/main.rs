mod api;
mod model;
mod repository;

use actix_cors::Cors;
use api::endpoints::{get_trades, get_ohlc, get_symbols, get_web, close_websocket};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use repository::postgresdb::PostgresRepository;
use repository::websocket::Websocket;
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

    let pool_backend = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:password@localhost:5432/db")
        .await
        .unwrap();

    let pool_websocket = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://admin:password@localhost:5432/db")
        .await
        .unwrap();

    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::permissive(); // should not be used in production code 

        let postgres_repository = PostgresRepository::init(pool_backend.clone());
        let postgres_data = Data::new(postgres_repository);
        
        let postgres_repository = PostgresRepository::init(pool_websocket.clone());
        let websocket_repository = Websocket::init(postgres_repository);
        let websocket_data = Data::new(websocket_repository);

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(postgres_data)
            .app_data(websocket_data)
            .service(get_trades)
            .service(get_ohlc)
            .service(get_symbols)
            .service(get_web)
            .service(close_websocket)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
