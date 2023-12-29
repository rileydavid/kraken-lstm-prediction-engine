use crate::routes::symbol::handlers::get_symbols;
use crate::utils::app_state::AppState;

use axum::routing::get;
use axum::Router;

pub async fn router(app_state: AppState) -> Router {
    Router::new()
    .route("/symbols", get(get_symbols))
    .with_state(app_state)
}