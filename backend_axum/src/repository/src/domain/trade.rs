use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};

use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TradeModel {
    pub time: DateTime<Utc>,
    pub price: BigDecimal,
    pub volume: BigDecimal,
    pub side: String,
    pub order_type: String,
    pub symbol: String,
}

impl TradeModel {
    pub fn new(
        time: DateTime<Utc>,
        price: BigDecimal,
        volume: BigDecimal,
        side: String,
        order_type: String,
        symbol: String,
    ) -> TradeModel {
        TradeModel {
            time,
            price,
            volume,
            side,
            order_type,
            symbol,
        }
    }
}

impl TradeModel {
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

impl TradeModel {
    pub fn set_time(&mut self, time: DateTime<Utc>) {
        self.time = time;
    }

    pub fn set_price(&mut self, price: BigDecimal) {
        self.price = price;
    }

    pub fn set_volume(&mut self, volume: BigDecimal) {
        self.volume = volume;
    }

    pub fn set_side(&mut self, side: String) {
        self.side = side;
    }

    pub fn set_order_type(&mut self, order_type: String) {
        self.order_type = order_type;
    }

    pub fn set_symbol(&mut self, symbol: String) {
        self.symbol = symbol;
    }
}

impl From<PgRow> for TradeModel {
    fn from(value: PgRow) -> Self {
        TradeModel::new(
            value.get("time_"),
            value.get("price"),
            value.get("volume"),
            value.get("side"),
            value.get("order_type"),
            value.get("symbol"),
        )
    }
}
