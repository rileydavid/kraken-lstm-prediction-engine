use crate::routes::ohlc::handlers::{post_ohlc_day_range, post_ohlc_hour_range};
use axum::routing::post;
use axum::Router;
use sqlx::PgPool;

pub async fn router(pool: &PgPool) -> Router {
    Router::new()
        .route("/ohlc/hour/range", post(post_ohlc_hour_range))
        .route("/ohlc/day/range", post(post_ohlc_day_range))
        .with_state(pool.clone())
}
