use crate::routes;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::net::SocketAddr;
use utils::core::webserver_config::get_address;

pub async fn run() {
    let app = Router::new()
        .route("/ping", get(ping))
        .merge(routes::ohlc::routes::router().await)
        .merge(routes::symbol::routes::router().await)
        //.merge(routes::convert::routes::router().await)
        .merge(routes::trade::routes::router().await);

    let adress = SocketAddr::from(get_address().await);

    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub async fn ping() -> Result<String, StatusCode> {
    Ok("service works well".to_owned())
}
