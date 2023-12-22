use std::sync::Arc;

use crate::cache::cache_dto::OhlcCacheDto;
use crate::cache::cache_service;
use crate::ohlc::ohlc_dto::OhlcDto;
use chrono::DateTime;
use chrono::Utc;
use repository::repository::ohlc_repository;
use rustis::bb8::State;
use rustis::client;
use rustis::client::Client;
use tracing::error;
use tracing::info;
use utils::core::cache;
use utils::core::postgresdb::Tx;
use utils::core::postgresdb::TxAsync;
use utils::error::generic_error::GenericError;

pub async fn get_ohlc_hour_symbol_interval(
    symbol_id: i32,
    interval: i32,
    client: Arc<Client>
) -> Result<Vec<OhlcDto>, GenericError> {

    //TODO: does not always serve the correct data 
    // latest bucket is still aggregating and thefore the last timestamp can be incomplete  
    // fetch only the fully aggregated buckets!!
    let end_date = Utc::now();
    let start_date = end_date - chrono::Duration::hours(interval.into());

    match cache_service::has_ts(&client, symbol_id).await {
        true => {
            info!("getting ohlc data from cache");
            let result = cache_service::get_ohlc(&client, symbol_id, start_date, end_date, interval).await;
            info!("got ohlc data from cache");
            match result {
                Ok(data) => {
                   return Ok(data);
                }
                Err(err) => {
                    // do nothing here
                },
            }
        }
        false => {
            // do nothing here
            cache_service::init_ts_ohlc(&client, symbol_id).await?;
        },
    }

    let mut tx = Tx::begin().await;
    match ohlc_repository::get_ohlc_hour_symbol_interval(&mut tx, symbol_id, interval).await {
        Ok(data) => {
            Tx::commit(tx).await;
            
            let result: Vec<OhlcDto> = data.clone()
                .into_iter()
                .map(|model| OhlcDto::from(model))
                .collect();   
            
            info!("inserting ohlc data into cache");
            let _ = cache_service::insert_ohlc(&client, data).await;
            info!("inserted ohlc data into cache");

            Ok(result)
        }
        Err(err) => Err(err),
    }

}

pub async fn get_ohlc_day_symbol_interval(
    symbol_id: i32,
    interval: i32,
) -> Result<Vec<OhlcDto>, GenericError> {
    let mut tx = Tx::begin().await;
    match ohlc_repository::get_ohlc_day_symbol_interval(&mut tx, symbol_id, interval).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<OhlcDto> = data
                .into_iter()
                .map(|model: repository::domain::ohlc::OhlcModel| OhlcDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub async fn get_ohlc_hour_start_date(
    symbol_id: i32,
    interval: i32,
    start_date: DateTime<Utc>,
    client: Arc<Client>
) -> Result<Vec<OhlcDto>, GenericError> {

    let end_date = start_date;
    let start_date = start_date - chrono::Duration::hours(interval.into());

    info!("start_date: {}", start_date);
    info!("end_date: {}", end_date);

    match cache_service::has_ts(&client, symbol_id).await {
        true => {
            info!("getting ohlc data from cache");
            let result = cache_service::get_ohlc(&client, symbol_id, start_date, end_date, interval).await;
            info!("got ohlc data from cache");
            match result {
                Ok(data) => {
                   return Ok(data);
                }
                Err(err) => {
                    // do nothing here
                },
            }
        }
        false => {
            cache_service::init_ts_ohlc(&client, symbol_id).await?;
        },
    }

    let mut tx = Tx::begin().await;
    match ohlc_repository::get_ohlc_hour_start_date(&mut tx, symbol_id, interval, start_date).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<OhlcDto> = data.clone()
                .into_iter()
                .map(|model| OhlcDto::from(model))
                .collect();
        
            info!("inserting ohlc data into cache");
            let _ = cache_service::insert_ohlc(&client, data).await;
            info!("inserted ohlc data into cache");

            Ok(result)
        }
        Err(err) => Err(err),
    }
}

pub async fn get_ohlc_day_start_date(
    symbol_id: i32,
    interval: i32,
    start_date: DateTime<Utc>,
) -> Result<Vec<OhlcDto>, GenericError> {
    let mut tx = Tx::begin().await;
    match ohlc_repository::get_ohlc_day_start_date(&mut tx, symbol_id, interval, start_date).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<OhlcDto> = data
                .into_iter()
                .map(|model| OhlcDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}
