use std::sync::Arc;

use crate::symbol::symbol_dto::SymbolDto;
use cache::{repository::symbol_cache, domain::symbol::SymbolCacheModel};
use repository::repository::symbol_repository;
use rustis::client::Client;
use utils::error::generic_error::GenericError;
use rayon::prelude::*;

pub async fn get_symbols(pool: &sqlx::PgPool, redis: Arc<Client>) -> Result<Vec<SymbolDto>, GenericError> {
    
    match symbol_cache::get_symbols(&redis).await {
        Ok(data) => {
            let result: Vec<SymbolDto> = data
                .into_par_iter()
                .map(|model| SymbolDto::from(&model))
                .collect();
            return Ok(result);
        }
        Err(_) => {/*ignore as the error is already logged*/},
    };
    
    match symbol_repository::get_symbols(pool).await {
        Ok(data) => {
            let result: Vec<SymbolDto> = data.clone()
                .into_par_iter()
                .map(|model| SymbolDto::from(&model))
                .collect();
            
            let symbols: Vec<SymbolCacheModel> = data.into_par_iter().map(|model| SymbolCacheModel::new(model.get_id().to_owned(), model.get_symbol().to_owned())).collect(); 
            let _ = symbol_cache::update_symbols(&redis, symbols).await;

            Ok(result)
        }
        Err(err) => Err(err),
    }
}
