use crate::routes::symbol::handlers::get_symbols;
use axum::routing::get;
use axum::Router;

pub async fn router() -> Router {
    Router::new()
    .route("/symbols", get(get_symbols))
}
