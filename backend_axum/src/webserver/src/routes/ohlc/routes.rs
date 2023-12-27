use std::sync::Arc;

use crate::routes::ohlc::handlers::{
    get_ohlc_day_symbol_interval, get_ohlc_hour_symbol_interval, post_ohlc_day_start_date,
    post_ohlc_hour_start_date,post_ohlc_hour_range
};
use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::Router;
use rustis::client::Client;

pub async fn router(redis: Arc<Client>) -> Router {

    //TODO: remove interval request as these don't perform well at all -- use range instead

    Router::new()
        .route("/ohlc/hour/range", post(post_ohlc_hour_range))
        .route(
            "/ohlc/hour/:symbol/:interval",
            get(get_ohlc_hour_symbol_interval),
        )
        .with_state(redis.clone())
        .route(
            "/ohlc/day/:symbol/:interval",
            get(get_ohlc_day_symbol_interval),
        )
        .route("/ohlc/day/start", post(post_ohlc_day_start_date))
        .route("/ohlc/hour/start", post(post_ohlc_hour_start_date))
        .with_state(redis)
        
}
