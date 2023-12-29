use std::sync::Arc;

use rustis::client::Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: Arc<Client>
}

impl AppState {
    pub fn new(pool: PgPool, redis: Arc<Client>) -> Self {
        Self { pool, redis }
    }
}
