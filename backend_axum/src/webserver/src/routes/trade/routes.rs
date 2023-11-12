use crate::routes::trade::handlers::get_trade_symbol_interval;
use axum::routing::get;
use axum::Router;

pub async fn router() -> Router {
    Router::new().route("/trade/:symbol/:interval", get(get_trade_symbol_interval))
}
