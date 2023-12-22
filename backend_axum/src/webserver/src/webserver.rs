use crate::routes;
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use std::{net::SocketAddr, sync::Arc};
use tower_http::cors::{Any, CorsLayer};
use utils::core::webserver_config::get_address;
use rustis::{
    client::Client,
    commands::{GenericCommands, StringCommands},
};

#[derive(Clone)]
pub struct Testing {
    client: Arc<Client>,
}

impl Testing {
    pub fn new(client: Arc<Client>) -> Self {
        Testing { client }
    }
}

pub async fn run() {
    let redis = utils::core::cache::init_client().await;

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/ping", get(ping))
        .merge(routes::ohlc::routes::router(redis).await)
        .merge(routes::symbol::routes::router().await)
        .merge(routes::import::routes::router().await)
        .merge(routes::modelexecution::routes::router().await)
        .merge(routes::trade::routes::router().await)
        .layer(cors);

    let adress = SocketAddr::from(get_address().await);

    axum::Server::bind(&adress)
        .serve(app.into_make_service())
        .await
        .unwrap_or_else(|_| panic!("Server cannot launch."));
}

pub async fn ping() -> Result<String, StatusCode> {
    Ok("service works well".to_owned())
}
