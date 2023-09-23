mod api;
mod model;
mod repository;

use actix_cors::Cors;

use api::endpoints::{get_trades, get_ohlc, get_symbols, close_websocket, get_cached_trades, websocket, get_prediction /* get_cached_trades_prediction ,new_prediction */};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use repository::postgresdb::PostgresRepository;
use repository::collector::Collector;
use repository::cachemanager::CacheManager;

use sqlx::postgres::PgPoolOptions;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Backend");

    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var(
        "DATABASE_URL",
        "postgres://admin:password@172.1.0.10:5432/db",
    );

    std::env::set_var(
        "WEBSERVER_IP",
        "172.1.0.14",
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

        

    let cache_repository = CacheManager::new();

    // this seems to work
    let collector_repository = Collector::init(pool_websocket.clone(), cache_repository.clone());

    let handle_collector = collector_repository.clone();
    
    let _ = HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::permissive(); // should not be used in production code 

        // for postgres 
        let postgres_repository = PostgresRepository::init(pool_backend.clone());
        let postgres_data = Data::new(postgres_repository);

        // Currently not working because the webserver instantiates this 4 times 
        let collector_data = Data::new(collector_repository.clone());
        let cache_manager_data = Data::new(cache_repository.clone());

        App::new()
            .wrap(cors)
            .wrap(logger)
            .app_data(postgres_data)
            .app_data(collector_data)
            .app_data(cache_manager_data)
            .service(get_trades)
            .service(get_ohlc)
            .service(get_symbols)
            .service(get_cached_trades)
            .service(get_prediction)
            .service(close_websocket)
            //.service(new_prediction)
            .service(websocket)
            //.service(web::resource("/ws").route(web::get().to(websocket)))
    })
    .bind((std::env::var("WEBSERVER_IP").unwrap(), 8000))?
    .run()
    .await;

    // shutdown websocket thread not working 
    let _ = handle_collector.close_websocket();

    return Ok(());

}
