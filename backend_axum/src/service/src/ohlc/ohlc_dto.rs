
use chrono::{DateTime, Utc};
use repository::domain::ohlc::OhlcModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OhlcDto {
    bucket: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    high: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low: Option<f64>,
    close_price: f64,
    volume: f64,
    count: i32,
    symbol_id: i32,
}

impl OhlcDto {
    pub fn new(
        bucket: DateTime<Utc>,
        open_price: Option<f64>,
        high: Option<f64>,
        low: Option<f64>,
        close_price: f64,
        volume: f64,
        count: i32,
        symbol_id: i32,
    ) -> Self {
        OhlcDto {
            bucket,
            open_price,
            high,
            low,
            close_price,
            volume,
            count,
            symbol_id,
        }
    }
}


impl From<&OhlcModel> for OhlcDto {
    fn from(value: &OhlcModel) -> Self {
        OhlcDto::new(
            value.get_bucket().to_owned(),
            value.get_open_price().to_owned(),
            value.get_high().to_owned(),
            value.get_low().to_owned(),
            value.get_close_price().to_owned(),
            value.get_volume().to_owned(),
            value.get_count().to_owned(),
            value.get_symbol_id().to_owned()
        )
    }
}

impl From<&OhlcDto> for OhlcModel {
    fn from(value: &OhlcDto) -> Self {
        OhlcModel::new(
            value.bucket,
            value.open_price,
            value.high,
            value.low,
            value.close_price,
            value.volume,
            value.count,
            value.symbol_id,
        )
    }
}
