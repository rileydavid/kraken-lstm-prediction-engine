use bigdecimal::BigDecimal;
use chrono::{Utc, DateTime, TimeZone};
use repository::domain::{ohlc::OhlcModel, symbol};
use bigdecimal::{ToPrimitive, FromPrimitive};

use crate::ohlc::ohlc_dto::OhlcDto;

#[derive(Clone, Copy)]
pub enum OhlcLabel {
    ClosePrice,
    OpenPrice,
    High,
    Low,
    Volume,
    Count
}

impl OhlcLabel {
    pub fn as_str(&self) -> &'static str {
        match self {
            OhlcLabel::ClosePrice => "close_price",
            OhlcLabel::OpenPrice => "open_price",
            OhlcLabel::High => "high",
            OhlcLabel::Low => "low",
            OhlcLabel::Volume => "volume",
            OhlcLabel::Count => "count",
        }
    }
}

pub struct OhlcKeyDto {
    key: String, // ohlc:close_price:symbol_id
    value: f64, // value but some are just integer (count)
    label: OhlcLabel
    //ohlc_type: OhlcType // used for the key
}

impl OhlcKeyDto { 
    pub fn new(key: String, value: f64, label: OhlcLabel) -> Self {
        OhlcKeyDto {
            key: key,
            value: value,
            label: label,
        }
    }

    pub fn get_key(&self) -> &String {
        &self.key
    }   

    pub fn get_value(&self) -> &f64 {
        &self.value
    }

    pub fn get_label(&self) -> &OhlcLabel {
        &self.label
    }
}

pub struct OhlcCacheDto {
    timestamp: i64,
    symbol_id: i32,
    keys: Vec<OhlcKeyDto>,
}

impl OhlcCacheDto {

    pub fn new(timestamp: i64, symbol_id: i32, keys: Vec<OhlcKeyDto>) -> Self {
        OhlcCacheDto {
            timestamp: timestamp,
            symbol_id: symbol_id,
            keys: keys,
        }
    }

    pub fn get_timestamp(&self) -> &i64 {
        &self.timestamp
    }

    pub fn get_symbol_id(&self) -> &i32 {
        &self.symbol_id
    }

    pub fn get_keys(&self) -> &Vec<OhlcKeyDto> {
        &self.keys
    }
}

fn get_ohlc_key(symbol_id: i32, label: OhlcLabel) -> String {
    format!("ohlc:{}:{}", label.as_str(), symbol_id)
}

fn push_key(keys: &mut Vec<OhlcKeyDto>, symbol_id: i32, value: f64, label: OhlcLabel) {
    keys.push(OhlcKeyDto::new(
        get_ohlc_key(symbol_id, label),
        value,
        label,
    ));
}

impl From<OhlcModel> for OhlcCacheDto {
    fn from(value: OhlcModel) -> Self {
        let mut keys = Vec::with_capacity(6);
        let symbol_id = value.get_symbol_id().to_owned();

        push_key(&mut keys, symbol_id.clone(), value.get_close_price().to_owned().to_f64().unwrap(), OhlcLabel::ClosePrice);
        push_key(&mut keys, symbol_id.clone(), value.get_open_price().to_owned().unwrap().to_f64().unwrap(), OhlcLabel::OpenPrice);
        push_key(&mut keys, symbol_id.clone(), value.get_high().to_owned().unwrap().to_f64().unwrap(), OhlcLabel::High);
        push_key(&mut keys, symbol_id.clone(), value.get_low().to_owned().unwrap().to_f64().unwrap(), OhlcLabel::Low);
        push_key(&mut keys, symbol_id.clone(), value.get_volume().to_owned().to_f64().unwrap(), OhlcLabel::Volume);
        push_key(&mut keys, symbol_id, value.get_count().to_owned() as f64, OhlcLabel::Count);

        OhlcCacheDto::new(
            datetime_to_timestamp(value.get_bucket().to_owned()),   
            value.get_symbol_id().to_owned(),
            keys
        )
    }
}

impl From<OhlcCacheDto> for OhlcDto {
    fn from(cache: OhlcCacheDto) -> Self {
        let mut open_price = None;
        let mut high = None;
        let mut low = None;
        let mut close_price = BigDecimal::from(0);
        let mut volume = BigDecimal::from(0);
        let mut count = 0;

        for key_dto in cache.get_keys() {
            match key_dto.get_label() {
                OhlcLabel::OpenPrice => open_price = FromPrimitive::from_f64(*key_dto.get_value()),
                OhlcLabel::High => high = FromPrimitive::from_f64(*key_dto.get_value()),
                OhlcLabel::Low => low = FromPrimitive::from_f64(*key_dto.get_value()),
                OhlcLabel::ClosePrice => close_price = FromPrimitive::from_f64(*key_dto.get_value()).unwrap(),
                OhlcLabel::Volume => volume = FromPrimitive::from_f64(*key_dto.get_value()).unwrap(),
                OhlcLabel::Count => count = *key_dto.get_value() as i32,
            }
        }

        OhlcDto::new(
            timestamp_to_datetime(cache.get_timestamp().to_owned()),
            open_price,
            high,
            low,
            close_price,
            volume,
            count,
            cache.get_symbol_id().to_owned(),
        )
    }
}



// TODO: issue with ohlc --> hour and day both have one timestamp that matches up?
// seperation of ohlc and ohlc_hour/ohlc_day
/* 
pub struct CacheOhlcDto {
    timestamp: i64,
    keys: Vec<OhlcKeyDto>,
    //close_price: CacheKeyDto,
    //open_price: CacheKeyDto,
    //high: CacheKeyDto,
    //low: CacheKeyDto,
    //volume: CacheKeyDto, 
    //count: CacheKeyDto,
    symbol_id: i32,
}


impl CacheOhlcDto {
    pub fn new(
        timestamp: i64,
        close_price: BigDecimal,
        open_price: BigDecimal,
        high: BigDecimal,
        low: BigDecimal,
        volume: BigDecimal,
        count: i32,
        symbol_id: i32, // used for the key
        //ohlc_type: OhlcType,
    ) -> Self {
        CacheOhlcDto {
            timestamp: timestamp,
            close_price: CacheKeyDto::new(format!("ohlc:close_price:{}", symbol_id), close_price.to_f64().unwrap()),
            open_price: CacheKeyDto::new(format!("ohlc:open_price:{}", symbol_id), open_price.to_f64().unwrap()),
            high: CacheKeyDto::new(format!("ohlc:high:{}", symbol_id), high.to_f64().unwrap()),
            low: CacheKeyDto::new(format!("ohlc:low:{}", symbol_id), low.to_f64().unwrap()),
            volume: CacheKeyDto::new(format!("ohlc:volume:{}", symbol_id), volume.to_f64().unwrap()),
            count: CacheKeyDto::new(format!("ohlc:count:{}", symbol_id), count as f64),
            symbol_id: symbol_id,
        }
    }

    pub fn get_timestamp(&self) -> &i64 {
        &self.timestamp
    }

    // create getters for each key
    pub fn get_close_price(&self) -> &CacheKeyDto {
        &self.close_price
    }

    pub fn get_open_price(&self) -> &CacheKeyDto {
        &self.open_price
    }

    pub fn get_high(&self) -> &CacheKeyDto {
        &self.high
    }

    pub fn get_low(&self) -> &CacheKeyDto {
        &self.low
    }

    pub fn get_volume(&self) -> &CacheKeyDto {
        &self.volume
    }

    pub fn get_count(&self) -> &CacheKeyDto {
        &self.count
    }

    pub fn get_cachekeydtos(&self) -> Vec<&CacheKeyDto> {
        let mut result: Vec<&CacheKeyDto> = Vec::new();

        result.push(&self.close_price);
        result.push(&self.open_price);
        result.push(&self.high);
        result.push(&self.low);
        result.push(&self.volume);
        result.push(&self.count);
            
        result
    }
}

impl From<OhlcModel> for CacheOhlcDto {
    fn from(value: OhlcModel) -> Self {
        CacheOhlcDto::new(
            datetime_to_timestamp(value.get_bucket().to_owned()),
            value.get_close_price().to_owned(),
            value.get_open_price().to_owned().unwrap(),
            value.get_high().to_owned().unwrap(),
            value.get_low().to_owned().unwrap(),
            value.get_volume().to_owned(),
            value.get_count().to_owned(),
            value.get_symbol_id().to_owned(),
        )
    }
}

impl From<CacheOhlcDto> for OhlcDto {
    fn from(value: CacheOhlcDto) -> Self {
        OhlcDto::new(
            timestamp_to_datetime(value.timestamp),
            Some(FromPrimitive::from_f64(value.open_price.value).unwrap()),
            Some(FromPrimitive::from_f64(value.high.value).unwrap()),
            Some(FromPrimitive::from_f64(value.low.value).unwrap()),
            FromPrimitive::from_f64(value.close_price.value).unwrap(),
            FromPrimitive::from_f64(value.volume.value).unwrap(),
            value.count.value as i32,
            value.symbol_id, 
        )
    }
}
 */


// accurate to the second --> enough for ohlc hour/day/min
pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let datetime: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    datetime
}

// accurate to the second --> enough for ohlc hour/day/min
fn datetime_to_timestamp(datetime: DateTime<Utc>) -> i64 {
    datetime.timestamp()
}
