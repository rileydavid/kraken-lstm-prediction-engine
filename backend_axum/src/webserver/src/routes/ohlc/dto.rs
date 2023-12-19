use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OhlcRequestDto {
    pub symbol_id: i32,
    pub interval: i32,
    pub start_date: DateTime<Utc>
}