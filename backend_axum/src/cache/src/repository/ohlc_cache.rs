use core::time;
use std::{collections::HashMap, sync::Arc, time::SystemTime};

use bigdecimal::{FromPrimitive, BigDecimal};
use chrono::{DateTime, Timelike, Utc, TimeZone, Date};


use repository::domain::{ohlc::{self, OhlcModel}, symbol};
use rustis::{
    client::Client,
    commands::{
        GenericCommands, StringCommands, TimeSeriesCommands, TsAddOptions, TsCreateOptions,
        TsDuplicatePolicy, TsGetOptions, TsGroupByOptions, TsMRangeOptions, TsRangeOptions,
        TsRangeSample,
    },
    resp::CollectionResponse,
};

use utils::{
    core::cache,
    error::{
        generic_error::{ErrorBody, GenericError},
        webserver_error::WebError,
    },
};

use tracing::error;
use tracing::info;

use crate::domain::ohlc::{OhlcCacheDto, OhlcKeyDto, timestamp_to_datetime};

// Maybe move all this to it's own module --> similar to repository

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

pub async fn insert_ohlc(
    client: &Arc<Client>,
    data: Vec<OhlcModel>,
) -> Result<(), GenericError> {
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

pub async fn get_ohlc(client: &Arc<Client>, symbol_id: i32, start_date: DateTime<Utc>, end_date: DateTime<Utc>, interval: i32) -> Result<Vec<OhlcModel>, GenericError> {

    let mut master_map: HashMap<String, HashMap<u64, f64>> = HashMap::new();

    for label in LABELS_OHLC.iter() {
        let key = "ohlc:".to_owned() + label + ":" + &symbol_id.to_string();
        let result: Result<Vec<(u64, f64)>, rustis::Error> = client
            .ts_range(key, start_date.timestamp()-1, end_date.timestamp()+1, TsRangeOptions::default())
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
                .and_then(|v| FromPrimitive::from_f64(*v)).unwrap();

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
                symbol_id
            );
            result.push(ohlc_dto);
        }
    }

    //TODO: make sure this logic is also correct for get_ohlc_hour without the fixed start_date
    // data might not be correct because the latest timestamp is still being aggregated
    if result.len() != (interval+1) as usize {
        info!("result len {}", result.len());
        info!("interval {}", interval);
        error!("Failed to get ohlc data from cache: not enough data");
        return Err(WebError::general_error("Failed to get ohlc data from cache: not enough data".to_string()));
    }

    Ok(result)
}


// accurate to the second --> enough for ohlc hour/day/min
/*
fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let datetime: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    datetime
}
 */



/*
        match result {
            Ok(_) => return Ok(()),
            Err(err) => {
                error!("Failed to create time series")
                Err(WebError::general_error(err.to_string()))
            }
        }
*/

/*
pub async fn is_cached(client: &Arc<Client>, symbol_id: i32, interval: i32) -> bool {
    // TODO: Test this!
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    let datetime = datetime
        .with_minute(0)
        .unwrap()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap();
    let window_end = datetime.timestamp();
    let window_start = window_end - ((interval as i64) * 60);

    let key_end = "ohlc:window_end:".to_owned() + &symbol_id.to_string();
    let key_start = "ohlc:window_start:".to_owned() + &symbol_id.to_string();

    let result_end: Option<String> = client.get(&key_end).await.unwrap();
    let result_start: Option<String> = client.get(&key_start).await.unwrap();

    if result_end.is_some() && result_start.is_some() {
        let end_timestamp = result_end.unwrap().parse::<i64>().unwrap();
        let start_timestamp = result_start.unwrap().parse::<i64>().unwrap();

        return end_timestamp == window_end && start_timestamp == window_start;
    }

    return false;
}

pub async fn update_cached_flag(
    client: &Arc<Client>,
    symbol_id: i32,
    window_start: i64,
    window_end: i64,
) -> Result<(), GenericError> {
    let key_end = "ohlc:window_end:".to_owned() + &symbol_id.to_string();
    let key_start = "ohlc:window_start:".to_owned() + &symbol_id.to_string();

    let result_end = client.set(&key_end, window_end).await;
    let result_start = client.set(&key_start, window_start).await;

    if result_end.is_err() || result_start.is_err() {
        return Err(WebError::general_error(
            "Failed to update cache flag".to_owned(),
        ));
    }

    Ok(())
}

pub async fn update(client: &Arc<Client>, value: Vec<CacheOhlcDto>) -> Result<(), GenericError> {
    for ohlc_dto in value {
        let timestamp = ohlc_dto.get_timestamp();
        let cache_key_dtos = ohlc_dto.get_cachekeydtos();

        for cache_key_dto in cache_key_dtos {
            let result = add(client.clone(), cache_key_dto, timestamp.clone()).await;
            match result {
                Ok(_) => {}
                Err(err) => {
                    return Err(err);
                }
            }
        }
    }
    Ok(())
}

pub async fn get_start_date(
    client: &Arc<Client>,
    symbol_id: i32,
    interval: i32,
    start_date: DateTime<Utc>,
) -> Result<f64, GenericError> {
    let ts_mrange_option: TsMRangeOptions = TsMRangeOptions::default();
    let ts_group_by_option: TsGroupByOptions = TsGroupByOptions::default();

    let start_unix = start_date.timestamp();
    let end_unix: i64 = start_date.timestamp() + ((interval as i64) * 60);

    let filters: Vec<String> = vec!["close_price=close_price".to_owned()];

    let result: Vec<TsRangeSample> = client
        .ts_mrange(
            start_unix,
            end_unix,
            ts_mrange_option,
            filters,
            ts_group_by_option,
        )
        .await
        .unwrap();

    info!("result {:?}", result);

    Ok(0.0)
}

pub async fn get_interval(
    client: &Arc<Client>,
    symbol_id: i32,
    interval: i32,
) -> Result<f64, GenericError> {
    let ts_get_option: TsGetOptions = TsGetOptions::default();
    let result: Option<(u64, f64)> = client
        .ts_get("ohlc:close_price:1", ts_get_option)
        .await
        .unwrap();

    match result {
        Some(data) => {
            info!("Successfully retrieved close price");
            Ok(data.1)
        }
        None => {
            info!("Failed to retrieve close price");
            Err(WebError::general_error(
                "Failed to retrieve close price".to_owned(),
            ))
        }
    }
}

pub async fn create_ts(client: Arc<Client>) -> Result<(), GenericError> {
    let mut ts_create_option =
        TsCreateOptions::default().duplicate_policy(TsDuplicatePolicy::First);
    ts_create_option = ts_create_option.labels(vec![("type".to_owned(), "close_price".to_owned())]);

    let result = client.ts_create("ohlc:1 ", ts_create_option).await.unwrap();

    Ok(())
}

async fn add(client: Arc<Client>, value: &CacheKeyDto, timestamp: i64) -> Result<(), GenericError> {
    let mut ts_add_option = TsAddOptions::default();

    //let labels: Vec<(String, String)> = vec![("close_price".to_owned(), "close_price".to_owned())];
    //ts_add_option = ts_add_option.labels(labels);
    //ts_add_option = ts_add_option.on_duplicate(TsDuplicatePolicy::First);

    info!("key {}", value.get_key());

    let result = client
        .ts_add(
            value.get_key(),
            timestamp,
            value.get_value().to_owned(),
            ts_add_option,
        )
        .await;

    match result {
        Ok(_) => {
            info!("Successfully updated close price");
            Ok(())
        }
        Err(err) => {
            info!("Failed to update close price");
            Err(WebError::general_error(err.to_string()))
        }
    }
}


async fn madd(client: Arc<Client>, value: Vec<CacheOhlcDto>) -> Result<(), GenericError> {

    // what i need is tuples of (key timestamp, value)
    Ok(())
}

async fn add_updated(client: Arc<Client>, value: &CacheKeyDto, timestamp: i64, ohlc_type: OhlcType) -> Result<(), GenericError> {
    let mut ts_add_option = TsAddOptions::default()
        .on_duplicate(TsDuplicatePolicy::First)
        .labels(vec![("type".to_owned(), ohlc_type.as_str().to_owned())]);

    //let labels: Vec<(String, String)> = vec![("close_price".to_owned(), "close_price".to_owned())];
    //ts_add_option = ts_add_option.labels(labels);
    //ts_add_option = ts_add_option.on_duplicate(TsDuplicatePolicy::First);

    info!("key {}", value.get_key());

    let result = client
        .ts_add(
            value.get_key(),
            timestamp,
            value.get_value().to_owned(),
            ts_add_option,
        )
        .await;

    match result {
        Ok(_) => {
            info!("Successfully updated close price");
            Ok(())
        }
        Err(err) => {
            info!("Failed to update close price");
            Err(WebError::general_error(err.to_string()))
        }
    }
}
 */
