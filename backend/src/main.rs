mod api;
mod model;
mod repository;

use actix_cors::Cors;
use api::endpoints::{get_trades, get_ohlc, get_symbols};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use repository::postgresdb::PostgresRepository;
use repository::websocket::Collector;
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Backend");

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var(
        "DATABASE_URL",
        "postgres://admin:password@localhost:5432/db",
    );

    std::env::set_var(
        "WEBSERVER_IP",
        "127.0.0.1",
    );

    env_logger::init();

    let pool_backend = PgPoolOptions::new()
        .max_connections(2)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    let pool_websocket = PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // this seems to work
    let websocket_repository = Collector::init(pool_websocket.clone());
    
    HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::permissive(); // should not be used in production code 

        let postgres_repository = PostgresRepository::init(pool_backend.clone());
        let postgres_data = Data::new(postgres_repository);

        // Currently not working because the webserver instantiates this 4 times 
        let websocket_data = Data::new(websocket_repository.clone());
     
        App::new()
            .wrap(cors)
            .wrap(logger)
            //.wrap(websocket_repository)
            //.app_data(websocket_data)
            .app_data(postgres_data)
            .app_data(websocket_data)
            .service(get_trades)
            .service(get_ohlc)
            .service(get_symbols)
            //.service(get_web)
            //.service(close_websocket)
    })
    .bind((std::env::var("WEBSERVER_IP").unwrap(), 8000))?
    .run()
    .await
}
