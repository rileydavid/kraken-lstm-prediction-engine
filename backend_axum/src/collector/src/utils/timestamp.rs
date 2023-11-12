use chrono::{DateTime, TimeZone, Utc};

pub fn parse_timestamp(timestamp: &str) -> DateTime<Utc> {
    let epoch_seconds: f64 = timestamp.parse().expect("Failed to parse epoch seconds");
    Utc.timestamp_opt(
        epoch_seconds as i64,
        (epoch_seconds.fract() * 1_000_000.0) as u32,
    )
    .unwrap()
}
