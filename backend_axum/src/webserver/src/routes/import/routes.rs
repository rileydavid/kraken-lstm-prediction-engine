
use crate::routes::import::handlers::import;
use axum::Router;
use axum::routing::get;

pub async fn router() -> Router {
    Router::new()
    .route("/import/:file_name/:symbol", get(import))
}
