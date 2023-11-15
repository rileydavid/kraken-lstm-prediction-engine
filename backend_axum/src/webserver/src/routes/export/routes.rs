
use crate::routes::export::handlers::export;
use axum::Router;
use axum::routing::get;

pub async fn router() -> Router {
    Router::new()
    .route("/export/:symbol", get(export))
}
