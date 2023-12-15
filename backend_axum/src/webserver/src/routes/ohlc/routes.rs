use crate::routes::ohlc::handlers::{
    get_ohlc_day_symbol_interval, get_ohlc_hour_symbol_interval, post_ohlc_day_start_date,
    post_ohlc_hour_start_date,
};
use axum::routing::{get, post};
use axum::Router;

pub async fn router() -> Router {
    Router::new()
        .route(
            "/ohlc/hour/:symbol/:interval",
            get(get_ohlc_hour_symbol_interval),
        )
        .route(
            "/ohlc/day/:symbol/:interval",
            get(get_ohlc_day_symbol_interval),
        )
        .route("/ohlc/day/start", post(post_ohlc_day_start_date))
        .route("/ohlc/hour/start", post(post_ohlc_hour_start_date))
}
