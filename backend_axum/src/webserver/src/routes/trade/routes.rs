use crate::routes::trade::handlers::get_trades_range;
use axum::routing::post;
use axum::Router;
use sqlx::PgPool;

pub async fn router(pool: &PgPool) -> Router {
    Router::new().route("/trades", post(get_trades_range))
    .with_state(pool.clone())
}
