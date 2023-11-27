use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use repository::domain::ohlc::OhlcModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OhlcDto {
    bucket: DateTime<Utc>,
    open_price: BigDecimal,
    high: BigDecimal,
    low: BigDecimal,
    close_price: BigDecimal,
    volume: BigDecimal,
    count: i32,
    symbol: String,
}

impl OhlcDto {
    pub fn new(
        bucket: DateTime<Utc>,
        open_price: BigDecimal,
        high: BigDecimal,
        low: BigDecimal,
        close_price: BigDecimal,
        volume: BigDecimal,
        count: i32,
        symbol: String,
    ) -> Self {
        OhlcDto {
            bucket,
            open_price,
            high,
            low,
            close_price,
            volume,
            count,
            symbol,
        }
    }
}

impl From<OhlcModel> for OhlcDto {
    fn from(value: OhlcModel) -> Self {
        OhlcDto::new(
            value.get_bucket().to_owned(),
            value.get_open_price().to_owned(),
            value.get_high().to_owned(),
            value.get_low().to_owned(),
            value.get_close_price().to_owned(),
            value.get_volume().to_owned(),
            value.get_count().to_owned(),
            value.get_symbol().to_owned()
        )
    }
}

impl From<OhlcDto> for OhlcModel {
    fn from(value: OhlcDto) -> Self {
        OhlcModel::new(
            value.bucket,
            value.open_price,
            value.high,
            value.low,
            value.close_price,
            value.volume,
            value.count,
            value.symbol,
        )
    }
}
