use chrono::{Utc, DateTime, TimeZone};
use repository::domain::ohlc::OhlcModel;
use bigdecimal::ToPrimitive;

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

// accurate to the second --> enough for ohlc hour/day/min
pub fn timestamp_to_datetime(timestamp: i64) -> DateTime<Utc> {
    let datetime: DateTime<Utc> = Utc.timestamp_opt(timestamp, 0).unwrap();
    datetime
}

// accurate to the second --> enough for ohlc hour/day/min
fn datetime_to_timestamp(datetime: DateTime<Utc>) -> i64 {
    datetime.timestamp()
}
