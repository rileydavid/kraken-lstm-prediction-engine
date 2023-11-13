
use crate::routes::convert::handlers::convert;
use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::Router;

pub async fn router() -> Router {
    Router::new()
    .route("/convert/:file_name/:currencypair", get(convert))
}
