use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelConfigModel {
    id: i32,
    symbol_id: i32,
    modelname: String, 
    data_type: String, 
    lookback: i32,
    lags: Vec<i32>,
    window_sizes: Vec<i32>,
    span_sizes: Vec<i32>,
    active: bool,
    features: Vec<String>
}

impl ModelConfigModel {
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
        features: Vec<String>
    ) -> ModelConfigModel {
        ModelConfigModel {
            id,
            symbol_id,
            modelname, 
            data_type,
            lookback,
            lags,
            window_sizes,
            span_sizes,
            active,
            features
        }
    }
}

impl ModelConfigModel {
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_symbol_id(&self) -> &i32 {
        &self.symbol_id
    }

    pub fn get_modelname(&self) -> &str {
        &self.modelname[..]
    }

    pub fn get_data_type(&self) -> &str { 
        &self.data_type[..]
    } 

    pub fn get_lookback(&self) -> &i32 {
        &self.lookback
    }

    pub fn get_lags(&self) -> &Vec<i32> {
        &self.lags
    }

    pub fn get_window_sizes(&self) -> &Vec<i32> {
        &self.window_sizes
    }

    pub fn get_span_sizes(&self) -> &Vec<i32> {
        &self.span_sizes
    }

    pub fn is_active(&self) -> &bool {
        &self.active
    }

    pub fn get_features(&self) -> &Vec<String> {
        &self.features
    }
}

impl From<PgRow> for ModelConfigModel {
    fn from(value: PgRow) -> Self {
        ModelConfigModel::new(
            value.get("id"),
            value.get("symbol_id"),
            value.get("modelname"),
            value.get("data_type"),
            value.get("lookback"),
            value.get("lags"),
            value.get("window_sizes"),
            value.get("span_sizes"),
            value.get("active"),
            value.get("features")
        )
    }
}
