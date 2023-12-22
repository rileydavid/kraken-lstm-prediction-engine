use std::{env, sync::Arc};

use rustis::{
    client::Client,
    commands::{GenericCommands, StringCommands, TimeSeriesCommands},
};
use tracing::info;

use crate::error::webserver_error::WebError;

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


/*
init connection --> also inits the TS
- create a new TS: TS CREATE ohlc  
*/
/*
pub async fn update(
    redis: Arc<Client>,
    ohcl: RedisOhlc,
) -> Result<(), WebError> {

    

    /*
    if value.is_none() {
        return Err(WebError::from(
            StatusCode::BAD_REQUEST,
            "Value not provided",
        ));
    }
    redis.set(key, value).await?;
     */
    Ok(())

}

 */
/*
async fn read(
    redis_client: Arc<Client>,
) -> Result<String, WebError> {
    let value: Option<String> = redis.get(&key).await?;
    value.ok_or_else(|| {
        ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Key `{key}` does not exist."),
        )
    })
}
 */
/*
async fn update(
    State(redis): State<Arc<Client>>,
    Path(key): Path<String>,
    value: Option<String>,
) -> Result<(), ServiceError> {
    if value.is_none() {
        return Err(ServiceError::new(
            StatusCode::BAD_REQUEST,
            "Value not provided",
        ));
    }
    redis.set(key, value).await?;
    Ok(())
}

async fn del(
    State(redis): State<Arc<Client>>,
    Path(key): Path<String>,
) -> Result<(), ServiceError> {
    let deleted = redis.del(&key).await?;
    if deleted > 0 {
        Ok(())
    } else {
        Err(ServiceError::new(
            StatusCode::NOT_FOUND,
            format!("Key `{key}` does not exist."),
        ))
    }
}
 */
/*

pub async fn set_key_value(key: &str, value: &str) {
    let mut connection = get_connection().await.unwrap();
    let result: String = connection.set(key, value).await.unwrap();
}

pub async fn get_key(key: &str) -> Option<String> {
    let mut connection = get_connection().await.unwrap();
    let result: Option<String> = match connection.get(key).await {
        Ok(data) => Some(data),
        Err(err) => None
    };
    result
}

pub async fn del_key(key: &str) -> Option<i32> {
    let mut connection = get_connection().await.unwrap();
    let result: Option<i32> = match connection.del::<&str, i32>(key).await {
        Ok(data) => Some(data),
        Err(err) => None
    };
    result
}
 */
/*
async fn health_check(pool: Pool<RedisConnectionManager>) {
    let mut connection = pool.get().await.unwrap();
    info!("execute : test cache connection ...");

    let health_check_key = "health_check_key";
    let health_check_value = "health_check_value";
    let _ : () = connection.set(health_check_key, health_check_value).await.unwrap();

    let result: String = connection.get(health_check_key).await.unwrap();
    connection.del::<&str, i32>(health_check_key).await.unwrap();
    assert_eq!(result, health_check_value);
    info!("executed: test cache connection");
}

static CONN: OnceCell<Pool<RedisConnectionManager>> = OnceCell::const_new();

pub async fn get_connection() -> Result<PooledConnection<'static,RedisConnectionManager>, GenericError> {
    (CONN.get_or_init(init_connection).await).get().await
        .map_err(|err| WebError::failed_getting_db_connection(err.to_string()))
}

pub async fn set_key_value(key: &str, value: &str) {
    let mut connection = get_connection().await.unwrap();
    let result: String = connection.set(key, value).await.unwrap();
}

pub async fn get_key(key: &str) -> Option<String> {
    let mut connection = get_connection().await.unwrap();
    let result: Option<String> = match connection.get(key).await {
        Ok(data) => Some(data),
        Err(err) => None
    };
    result
}

pub async fn del_key(key: &str) -> Option<i32> {
    let mut connection = get_connection().await.unwrap();
    let result: Option<i32> = match connection.del::<&str, i32>(key).await {
        Ok(data) => Some(data),
        Err(err) => None
    };
    result
}
 */

/*
    OHLC

    symbol:1 value (ETHUSD)

    TS.ADD ohlc:1:close_price timestamp value
    TS.ADD ohlc:1:high timestamp value
    TS.ADD ohlc:1:low timestamp value
    TS.ADD ohlc:1:open_price timestamp value
    TS.ADD ohlc:1:volume timestamp value
    TS.ADD ohlc:1:count timestamp value


    TS.ADD ohlc:1:close_price timestamp value RETENTION time_to_live

    # fetching data in given range
    TS.RANGE ohlc:1:close_price from_timestamp to_timestamp
*/
