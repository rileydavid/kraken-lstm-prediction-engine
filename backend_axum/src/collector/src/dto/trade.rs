use std::str::FromStr;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use repository::domain::trade::TradeModel;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::timestamp::parse_timestamp;

#[derive(Serialize, sqlx::FromRow, Deserialize, Debug, Clone)]
pub struct Trade {
    //pub channel_id: i32,
    time: DateTime<Utc>,
    price: BigDecimal,
    volume: BigDecimal,
    side: String,
    order_type: String,
    symbol: String,
}

impl Trade {
    pub fn get_time(&self) -> &DateTime<Utc> {
        &self.time
    }
    pub fn get_price(&self) -> &BigDecimal {
        &self.price
    }
    pub fn get_volume(&self) -> &BigDecimal {
        &self.volume
    }
    pub fn get_side(&self) -> &str {
        &self.side[..]
    }
    pub fn get_order_type(&self) -> &str {
        &self.order_type[..]
    }
    pub fn get_symbol(&self) -> &str {
        &self.symbol[..]
    }
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

    pub fn parse_from_json(json: Value) -> Option<Vec<Trade>> {
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
                        parse_timestamp(trade_array.get(2).unwrap().as_str().unwrap()),
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
        Some(trades)
    }
}

impl From<Trade> for TradeModel {
    fn from(value: Trade) -> Self {
        TradeModel::new(
            value.get_time().to_owned(),
            value.get_price().to_owned(),
            value.get_volume().to_owned(),
            value.get_side().to_owned(),
            value.get_order_type().to_owned(),
            value.get_symbol().to_owned(),
        )
    }
}
