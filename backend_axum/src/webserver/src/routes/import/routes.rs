
use crate::routes::import::handlers::{import, upload};
use axum::Router;
use axum::extract::DefaultBodyLimit;
use axum::routing::{get, post};
use sqlx::PgPool;

const MAX_CONTENT_LENGTH: usize = 2000 * 1024 * 1024; // 2000MB in bytes

pub async fn router(pool: &PgPool) -> Router {
    Router::new()
    .route("/import/:file_name/:symbol", get(import))
    .route("/upload", post(upload))
    .layer(DefaultBodyLimit::max(MAX_CONTENT_LENGTH))
    .with_state(pool.clone())
}
