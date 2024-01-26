use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::env;
use tokio::sync::OnceCell;
use tracing::info;

static CONN: OnceCell<Pool<Postgres>> = OnceCell::const_new();

pub async fn get_connection() -> &'static Pool<Postgres> {
    CONN.get_or_init(init_connection).await
}

async fn init_connection() -> Pool<Postgres> {
    info!("execute : initializing db connection ...");
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set!"));

    info!("DATABASE_URL: {}", db_url);

    let pool = PgPoolOptions::new()
        .max_connections(4)
        .connect(&db_url)
        .await
        .unwrap_or_else(|_| {
            panic!("Cannot connect to the database. Please check your configuration.")
        });
    info!("executed: initializing db connection");
    pool
}


