use bigdecimal::FromPrimitive;
use chrono::{DateTime, Utc};
use repository::domain::ohlc::OhlcModel;
use std::{collections::HashMap, sync::Arc};

use rustis::{
    client::Client,
    commands::{
        TimeSeriesCommands, TsAddOptions, TsCreateOptions, TsDuplicatePolicy, TsGetOptions,
        TsRangeOptions,
    },
};

use utils::error::{generic_error::GenericError, webserver_error::WebError};

use tracing::error;
use tracing::info;

use crate::domain::ohlc::{timestamp_to_datetime, OhlcCacheDto, OhlcKeyDto};

// switch all tables to use double precision
// --> better for timescale db (contious aggregates)
// --> no conversion needed for redis

// Issue how manage the cache?
// --> need to know when to update the cache

// no issue when getting certain date range (in the past) --> just make sure there is a timestamp each hour
// getting fromDate to current time is problematic as the current value is still being updated constantly

//
// idea check if values are in cache by checking these two keys
// key, value --> ohlc:window_end:1, end_timestamp
// key, value --> ohlc:window_start:1, start_timestamp

// Structure for keeping ohlc_hour data
// --> only doing hour for now because day is not used by any model

// key is the datatype and the symbol_id
// key: ohlc:1
// timestamp: i64 (unix timestamp)
// value: f64 (close_price, low, high, count, volume, open_price)
// labels: type=close_price, type=low, type=high, type=count, type=volume, type=open_price

// --> this enables fast filtering of data by type and symbol_id

// Manual testing commands:
// TS.create ohlc:close_price:1 DUPLICATE_POLICY first LABELS type close_price
// TS.create ohlc:open_price:1 DUPLICATE_POLICY first LABELS type open_price
// TS.ADD ohlc:close_price:1 1630000000 1.0
// TS.ADD ohlc:open_price:1 1630000000 2.0
// TS.MRANGE 1620000000 1630000001 WITHLABELS FILTER type=(close_price,open_price)

const LABELS_OHLC: [&str; 6] = [
    "close_price",
    "open_price",
    "high",
    "low",
    "volume",
    "count",
];

const CLOSE_PRICE: &str = "close_price";
const OPEN_PRICE: &str = "open_price";
const HIGH: &str = "high";
const LOW: &str = "low";
const VOLUME: &str = "volume";
const COUNT: &str = "count";

fn get_default_ts_create_options_ohlc() -> TsCreateOptions {
    TsCreateOptions::default()
        .retention(0)
        .duplicate_policy(TsDuplicatePolicy::First)
}

// doubled up on the duplicate policy but whatever
//const TS_ADD_OPTIONS: TsAddOptions = TsAddOptions::default()
//    .on_duplicate(TsDuplicatePolicy::First);

//const TS_FILTER_OHLC: [&str; 6] = ["type=close_price", "type=open_price", ("type", "high"), ("type", "low"), ("type", "volume"), ("type", "count")];

// Question: how do i know that a symbol arleady exists in the cache?
pub async fn init_ts_ohlc(client: &Arc<Client>, symbol_id: i32) -> Result<(), GenericError> {
    info!("Initializing time series for symbol {}", symbol_id);

    for label in LABELS_OHLC.iter() {
        let key = "ohlc:".to_owned() + label + ":" + &symbol_id.to_string();
        let options = get_default_ts_create_options_ohlc().labels(vec![
            ("type".to_owned(), label.to_owned()),
            ("symbol_id".to_owned(), &symbol_id.to_string()),
        ]);
        let result = client.ts_create(key, options).await;

        match result {
            Ok(_) => {}
            Err(err) => {
                error!("Failed to create time series");
                return Err(WebError::general_error(err.to_string()));
            }
        }
    }

    Ok(())
}

pub async fn has_ts(client: &Arc<Client>, symbol_id: i32) -> bool {
    info!("Checking if symbol {} has time series", symbol_id);

    let key = "ohlc:close_price".to_owned() + ":" + &symbol_id.to_string();

    match client.ts_get(&key, TsGetOptions::default()).await {
        Ok(_) => {
            info!("Symbol already exists in the cache");
            true
        }
        Err(_) => false,
    }
}

pub async fn insert_ohlc(client: &Arc<Client>, data: Vec<OhlcModel>) -> Result<(), GenericError> {
    info!("Inserting ohlc data into cache");

    let data_converted: Vec<OhlcCacheDto> = data
        .into_iter()
        .map(|model: repository::domain::ohlc::OhlcModel| OhlcCacheDto::from(model))
        .collect();

    for ohlc in data_converted {
        let timestamp = ohlc.get_timestamp();
        //let symbol_id = ohlc.get_symbol_id();
        let keys = ohlc.get_keys();

        for key in keys {
            let result = insert_ohlc_key(client.clone(), key, timestamp).await;
            match result {
                Ok(_) => {}
                Err(err) => {
                    error!("Failed to add ohlc data to cache");
                    return Err(err);
                }
            }
        }
    }

    Ok(())
}

async fn insert_ohlc_key(
    client: Arc<Client>,
    key_dto: &OhlcKeyDto,
    timestamp: &i64,
) -> Result<(), GenericError> {
    let result = client
        .ts_add(
            key_dto.get_key(),
            timestamp.to_owned(),
            key_dto.get_value().to_owned(),
            TsAddOptions::default().on_duplicate(TsDuplicatePolicy::First),
        )
        .await;

    match result {
        Ok(_) => {
            //info!("Successfully added ohlc data to cache");
            return Ok(());
        }
        Err(err) => {
            error!("Failed to add ohlc data to cache: {}", err);
            return Err(WebError::general_error(err.to_string()));
        }
    }
}

pub async fn get_ohlc(
    client: &Arc<Client>,
    symbol_id: i32,
    start_date: DateTime<Utc>,
    end_date: DateTime<Utc>,
    interval: i32,
) -> Result<Vec<OhlcModel>, GenericError> {
    let mut master_map: HashMap<String, HashMap<u64, f64>> = HashMap::new();

    for label in LABELS_OHLC.iter() {
        let key = "ohlc:".to_owned() + label + ":" + &symbol_id.to_string();
        let result: Result<Vec<(u64, f64)>, rustis::Error> = client
            .ts_range(
                key,
                start_date.timestamp() - 1,
                end_date.timestamp() + 1,
                TsRangeOptions::default(),
            )
            .await;

        match result {
            Ok(data) => {
                master_map.insert(label.to_string(), data.into_iter().collect());
            }
            Err(err) => {
                error!("Failed to get ohlc data from cache: {}", err);
                return Err(WebError::general_error(err.to_string()));
            }
        }
    }

    let mut result: Vec<OhlcModel> = Vec::new();

    if let Some(close_price_map) = master_map.get(CLOSE_PRICE) {
        for (key, value) in close_price_map {
            let close_price = FromPrimitive::from_f64(*value).unwrap();

            //TODO: improve this somehow
            // create function for this

            let open_price = master_map
                .get(OPEN_PRICE)
                .and_then(|m| m.get(key))
                .and_then(|v| FromPrimitive::from_f64(*v));

            let high = master_map
                .get(HIGH)
                .and_then(|m| m.get(key))
                .and_then(|v| FromPrimitive::from_f64(*v));

            let low = master_map
                .get(LOW)
                .and_then(|m| m.get(key))
                .and_then(|v| FromPrimitive::from_f64(*v));

            let volume = master_map
                .get(VOLUME)
                .and_then(|m| m.get(key))
                .and_then(|v| FromPrimitive::from_f64(*v))
                .unwrap();

            let count = master_map
                .get(COUNT)
                .and_then(|m| m.get(key))
                .map(|&v| v as i32);

            let time = timestamp_to_datetime(*key as i64);

            let ohlc_dto = OhlcModel::new(
                time,
                open_price,
                high,
                low,
                close_price,
                volume,
                count.unwrap_or_default(),
                symbol_id,
            );
            result.push(ohlc_dto);
        }
    }

    //TODO: make sure this logic is also correct for get_ohlc_hour without the fixed start_date
    // data might not be correct because the latest timestamp is still being aggregated
    if result.len() != (interval + 1) as usize {
        info!("result len {}", result.len());
        info!("interval {}", interval);
        error!("Failed to get ohlc data from cache: not enough data");
        return Err(WebError::general_error(
            "Failed to get ohlc data from cache: not enough data".to_string(),
        ));
    }

    Ok(result)
}

// TOOD: make use of this
#[allow(dead_code)]
fn get_value_from_map(
    master_map: &HashMap<String, HashMap<String, f64>>,
    key: &str,
    value_key: &str,
) -> Option<f64> {
    master_map
        .get(value_key)
        .and_then(|m| m.get(key))
        .and_then(|v| FromPrimitive::from_f64(*v))
}
