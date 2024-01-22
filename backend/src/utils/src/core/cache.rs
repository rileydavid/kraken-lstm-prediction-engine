use std::{env, sync::Arc};

use rustis::{
    client::Client,
    commands::{GenericCommands, StringCommands},
};
use tracing::info;

pub async fn init_client() -> Arc<Client> {
    info!("Initializing Redis Connection");
    let redis_url = env::var("REDIS_URL").unwrap_or_else(|_| panic!("REDIS_URL must be set!"));
    let client = Client::connect(redis_url)
        .await
        .unwrap_or_else(|_| panic!("Redis Client could not be initialized!"));

    health_check(&client).await;

    Arc::new(client)
}

async fn health_check(client: &Client) {
    info!("Testing Redis Connection");

    let health_check_key = "health_check_key";
    let health_check_value = "health_check_value";

    let _: () = client
        .set(health_check_key, health_check_value)
        .await
        .unwrap();
    let result: String = client.get(health_check_key).await.unwrap();

    client.del(health_check_key).await.unwrap();
    assert_eq!(result, health_check_value);

    info!("Redis successfully initialized");
}
