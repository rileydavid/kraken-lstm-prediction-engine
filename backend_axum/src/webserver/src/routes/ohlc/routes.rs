use crate::routes::ohlc::handlers::{get_ohlc_hour_symbol_interval, get_ohlc_day_symbol_interval};
use axum::routing::get;
use axum::Router;

pub async fn router() -> Router {
    Router::new()
    .route("/ohlc/hour/:symbol/:interval", get(get_ohlc_hour_symbol_interval))
    .route("/ohlc/day/:symbol/:interval", get(get_ohlc_day_symbol_interval))
}
