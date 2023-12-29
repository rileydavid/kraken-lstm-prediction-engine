use std::sync::Arc;

use rustis::{client::Client, commands::{JsonCommands, JsonGetOptions, SetCondition}};
use utils::error::{generic_error::GenericError, repository_error::RepositoryError};
use crate::domain::symbol::SymbolCacheModel;  

use tracing::info;

const SYMBOLS_KEY: &str = "symbols"; 

pub async fn get_symbols(redis: &Arc<Client>) -> Result<Vec<SymbolCacheModel>, GenericError> {
    
    let option = JsonGetOptions::default();

    let mut raw_string: String = redis.json_get(SYMBOLS_KEY, option).await.unwrap();
    
    if raw_string.len() == 0 {
        info!("cache miss");
        return Err(RepositoryError::general_error("cache miss".into()));
    };
    // result comes in [[]] removing outter shell of array
    raw_string = raw_string.as_str()[1..raw_string.len()-1].to_string();

    let result: Vec<SymbolCacheModel> = serde_json::from_str(&raw_string).unwrap();

    Ok(result)
}

pub async fn update_symbols(redis: &Arc<Client>, symbols: Vec<SymbolCacheModel>) {
    info!("updating symbols {:?}", symbols);
    let result = redis.json_set(SYMBOLS_KEY, "$".to_string(), serde_json::to_string(&symbols).unwrap(), SetCondition::None).await;
    info!("updated symbols {:?}", result);
}
