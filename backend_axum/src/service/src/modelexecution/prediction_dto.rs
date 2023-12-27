use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use repository::domain::model_config::ModelConfigModel;
use serde::{Deserialize, Serialize};

use crate::ohlc::ohlc_dto::OhlcDto;

#[derive(Serialize, Deserialize)]
pub struct PredictionRequestDto {
    id: i32,
    symbol_id: i32,
    modelname: String,
    data_type: String,
    lookback: i32,
    lags: Vec<i32>,
    window_sizes: Vec<i32>,
    span_sizes: Vec<i32>,
    active: bool,
    features: Vec<String>,
    from_date: Option<DateTime<Utc>>,
    data: Option<Vec<OhlcDto>>, // TODO: maybe it makes sense to send the data directly to the model service
}

impl PredictionRequestDto {
    pub fn new(
        id: i32,
        symbol_id: i32,
        modelname: String,
        data_type: String,
        lookback: i32,
        lags: Vec<i32>,
        window_sizes: Vec<i32>,
        span_sizes: Vec<i32>,
        active: bool,
        features: Vec<String>,
        from_date: Option<DateTime<Utc>>,
        data: Option<Vec<OhlcDto>>
    ) -> Self {
        PredictionRequestDto {
            id, 
            symbol_id,
            modelname,
            data_type,
            lookback,
            lags,
            window_sizes,
            span_sizes,
            active,
            features,
            from_date,
            data
        }
    }
}

impl PredictionRequestDto {
    pub fn set_from_date(&mut self, from_date: DateTime<Utc>) {
        self.from_date = Some(from_date);
    }

    pub fn set_data(&mut self, data: Vec<OhlcDto>) {
        self.data = Some(data);
    }
}

impl From<ModelConfigModel> for PredictionRequestDto {
    fn from(value: ModelConfigModel) -> Self {
        PredictionRequestDto::new(
            value.get_id().to_owned(),
            value.get_symbol_id().to_owned(),
            value.get_modelname().to_owned(),
            value.get_data_type().to_owned(),
            value.get_lookback().to_owned(),
            value.get_lags().to_owned(),
            value.get_window_sizes().to_owned(),
            value.get_span_sizes().to_owned(),
            value.is_active().to_owned(),
            value.get_features().to_owned(),
            None,
            None
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct PredictionResponseDto {
    bucket: DateTime<Utc>,
    close_price: BigDecimal,
}

impl PredictionResponseDto {
    pub fn new(bucket: DateTime<Utc>, close_price: BigDecimal) -> Self {
        PredictionResponseDto { bucket, close_price }
    }
}

