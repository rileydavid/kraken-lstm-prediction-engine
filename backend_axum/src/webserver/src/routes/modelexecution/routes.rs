use crate::routes::modelexecution::handlers::{post_execute_model, get_test};
use axum::routing::{post, get};
use axum::Router;

pub async fn router() -> Router {
    Router::new()
        .route("/execute/model", post(post_execute_model))
        .route("/test", get(get_test))
}
