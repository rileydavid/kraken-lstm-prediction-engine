use std::sync::Arc;

use rustis::client::Client;
use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub redis: Arc<Client>,
    pub sender: tokio::sync::mpsc::Sender<String>,
}

impl AppState {
    pub fn new(pool: PgPool, redis: Arc<Client>, sender: tokio::sync::mpsc::Sender<String>) -> Self {
        Self { pool, redis , sender}
    }
}
