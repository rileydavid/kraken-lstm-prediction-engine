use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ModelExecutionRequestDto {
    pub symbol_id: i32,
    pub from_date: DateTime<Utc>
}
