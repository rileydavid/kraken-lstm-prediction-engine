
use crate::routes::convert::handlers::file_upload;
use axum::extract::DefaultBodyLimit;
use axum::routing::post;
use axum::Router;

pub async fn router() -> Router {
    Router::new()
    .route("/upload", post(file_upload))
    .layer(DefaultBodyLimit::max(10240))
}
