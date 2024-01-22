
use crate::routes::upload::handlers::upload;
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::post;

const MAX_CONTENT_LENGTH: usize = 2000 * 1024 * 1024; // 2000MB in bytes

pub async fn router() -> Router {
    Router::new()
    .route("/upload", post(upload))
    .layer(DefaultBodyLimit::max(MAX_CONTENT_LENGTH))
}
