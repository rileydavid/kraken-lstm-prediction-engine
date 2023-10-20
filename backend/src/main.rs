mod api;
mod model;
mod repository;

use std::sync::Arc;

use dotenv::dotenv;

use actix_cors::Cors;

use api::endpoints::{get_trades, get_symbols, close_websocket, get_cached_trades, websocket, get_prediction /* get_cached_trades_prediction ,new_prediction */};
use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use log::info;
use repository::postgresdb::PostgresRepository;
use repository::collector::Collector;
use repository::cachemanager::CacheManager;

use sqlx::postgres::PgPoolOptions;


/*
 TODO: 
 * Struct for communication between all parts 
 *  
 * 
*/


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    // load env configuration --> DB URL, etc
    dotenv().ok();

    // init logging
    env_logger::init();
    info!("Starting Backend");
    
    // init postgres pool 
    let pgpool = Arc::new(PgPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap());

    
    let cache_repository = CacheManager::new();
    let postgres_repository = PostgresRepository::init(pgpool.clone());

    // this seems to work
    let collector_repository = Collector::init(pgpool.clone(), cache_repository.clone(), postgres_repository.clone());
    let handle_collector = collector_repository.clone();
    
    let _ = HttpServer::new(move || {
        let logger = Logger::default();
        let cors = Cors::permissive(); // should not be used in production code 

        // for postgres 
        let postgres_data = Data::new(postgres_repository.clone());

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
            //.service(get_ohlc)
            .service(get_symbols)
            .service(get_cached_trades)
            .service(get_prediction)
            .service(close_websocket)
            //.service(new_prediction)
            .service(websocket)
            //.service(web::resource("/ws").route(web::get().to(websocket)))
    })
    .workers(4)
    .bind((std::env::var("WEBSERVER_IP").unwrap(), 8000))?
    .run()
    .await;

    info!("here?");
    // shutdown websocket thread not working 
    let _ = handle_collector.close_websocket();

    return Ok(());
}
