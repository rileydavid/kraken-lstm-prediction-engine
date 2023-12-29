use crate::routes::modelexecution::handlers::post_execute_model;
use axum::routing::post;
use axum::Router;
use sqlx::PgPool;

pub async fn router(pool: &PgPool) -> Router {
    Router::new()
        .route("/execute/model", post(post_execute_model))
        .with_state(pool.clone())
}
