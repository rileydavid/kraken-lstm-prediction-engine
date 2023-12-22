use bigdecimal::{BigDecimal, FromPrimitive};
use cache::domain::ohlc::{OhlcCacheDto, OhlcLabel, timestamp_to_datetime};
use chrono::{DateTime, Utc};
use repository::domain::ohlc::OhlcModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct OhlcDto {
    bucket: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    open_price: Option<BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    high: Option<BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    low: Option<BigDecimal>,
    close_price: BigDecimal,
    volume: BigDecimal,
    count: i32,
    symbol_id: i32,
}

impl OhlcDto {
    pub fn new(
        bucket: DateTime<Utc>,
        open_price: Option<BigDecimal>,
        high: Option<BigDecimal>,
        low: Option<BigDecimal>,
        close_price: BigDecimal,
        volume: BigDecimal,
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
            value.get_symbol_id().to_owned()
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
            value.symbol_id,
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
