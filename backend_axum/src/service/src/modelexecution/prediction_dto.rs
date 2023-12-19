use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use repository::domain::model_config::ModelConfigModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    start_date: Option<DateTime<Utc>>,
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
        start_date: Option<DateTime<Utc>>,
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
            start_date,
        }
    }
}

impl PredictionRequestDto {
    pub fn set_start_date(&mut self, start_date: DateTime<Utc>) {
        self.start_date = Some(start_date);
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
            None
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct PredictionResponseDto {
    bucket: DateTime<Utc>,
    price: BigDecimal,
}

impl PredictionResponseDto {
    pub fn new(bucket: DateTime<Utc>, price: BigDecimal) -> Self {
        PredictionResponseDto { bucket, price }
    }
}

