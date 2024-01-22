use crate::{routes, utils::app_state::AppState};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use utils::core::{webserver_config::get_address, postgresdb};

pub async fn run(sender: tokio::sync::mpsc::Sender<String>) {

    //let redis = utils::core::cache::init_client().await;
    let redis = None; //currently this is not used

    let pool = postgresdb::get_connection().await; 
    let app_state = AppState::new(pool.clone(), redis, sender);

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ping", get(ping))
        .merge(routes::ohlc::routes::router(pool).await)
        .merge(routes::symbol::routes::router(app_state).await)
        .merge(routes::import::routes::router(pool).await)
        .merge(routes::modelexecution::routes::router(pool).await)
        .merge(routes::trade::routes::router(pool).await)
        .merge(routes::upload::routes::router().await)
        .layer(cors);

    let adress = SocketAddr::from(get_address().await);

    let listener = TcpListener::bind(adress).await.unwrap();

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub async fn ping() -> Result<String, StatusCode> {
    Ok("service works well".to_owned())
}
