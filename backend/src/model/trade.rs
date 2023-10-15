use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Serialize, sqlx::FromRow, Deserialize, Debug, Clone)]
pub struct Trade {
    //pub channel_id: i32,
    pub time: DateTime<Utc>,
    pub price: BigDecimal,
    pub volume: BigDecimal,
    pub side: String,
    pub order_type: String,
    pub symbol: String,
}

impl Trade {
    pub fn new(
        time: DateTime<Utc>,
        price: BigDecimal,
        volume: BigDecimal,
        side: String,
        order_type: String,
        symbol: String,
    ) -> Trade {
        Trade {
            time,
            price,
            volume,
            side,
            order_type,
            symbol,
        }
    }

    pub fn parse(json: Value) -> Option<Vec<Trade>> {
        let json_value = json.as_array().unwrap();
        let symbol = json_value
            .get(3)
            .unwrap()
            .as_str()
            .unwrap()
            .replace("/", "");
        let mut trades = Vec::new();

        //skipping channel id
        for value in json_value.iter().skip(1) {
            if value.is_array() {
                for array in value.as_array().unwrap() {
                    let trade_array = array.as_array().unwrap();
                    trades.push(Trade::new(
                        Self::parse_timestamp(trade_array.get(2).unwrap().as_str().unwrap()),
                        BigDecimal::from_str(trade_array.get(0).unwrap().as_str().unwrap())
                            .unwrap(),
                        BigDecimal::from_str(trade_array.get(1).unwrap().as_str().unwrap())
                            .unwrap(),
                        trade_array.get(3).unwrap().as_str().unwrap().to_string(),
                        trade_array.get(4).unwrap().as_str().unwrap().to_string(),
                        symbol.to_string(),
                    ));
                }
            }
        }
        return Some(trades);
    }

    pub fn to_json(&self) -> serde_json::Value {
        json!({
            "time": self.time.to_rfc3339(),
            "price": self.price.to_string(),
            "volume": self.volume.to_string(),
            "side": &self.side,
            "order_type": &self.order_type,
            "symbol": &self.symbol,
        })
    }

    fn parse_timestamp(timestamp: &str) -> DateTime<Utc> {
        let epoch_seconds: f64 = timestamp.parse().expect("Failed to parse epoch seconds");
        return Utc.timestamp_opt(epoch_seconds as i64, (epoch_seconds.fract() * 1_000_000.0) as u32).unwrap();
    }

}
