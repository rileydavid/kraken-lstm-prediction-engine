use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OhlcRequestDto {
    pub symbol_id: i32,
    pub interval: Option<i32>,
    pub from_date: DateTime<Utc>, 
    pub to_date: Option<DateTime<Utc>>,
}