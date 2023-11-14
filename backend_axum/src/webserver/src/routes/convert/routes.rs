
use crate::routes::convert::handlers::convert;
use axum::Router;
use axum::routing::get;

pub async fn router() -> Router {
    Router::new()
    .route("/convert/:file_name/:symbol", get(convert))
}
