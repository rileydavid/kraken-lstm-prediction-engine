
use crate::routes::import::handlers::import;
use axum::Router;
use axum::routing::get;
use sqlx::PgPool;

pub async fn router(pool: &PgPool) -> Router {
    Router::new()
    .route("/import/:file_name/:symbol", get(import))
    .with_state(pool.clone())
}
