use crate::routes::symbol::handlers::{get_symbols, get_add_symbol, get_available_symbols}; //get_add_symbol
use crate::utils::app_state::AppState;

use axum::routing::get;
use axum::Router;

// come up with better routes and names
pub async fn router(app_state: AppState) -> Router {
    Router::new()
    .route("/symbols", get(get_symbols))
    .route("/symbols/available", get(get_available_symbols))
    .route("/symbols/add/:symbol", get(get_add_symbol))
    .with_state(app_state)
}