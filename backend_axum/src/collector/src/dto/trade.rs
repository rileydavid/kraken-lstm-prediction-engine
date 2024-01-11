use std::fmt;

use chrono::{DateTime, Utc, TimeZone};
use repository::domain::trade::TradeModel;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Trade {
    #[serde(deserialize_with = "parse_string_to_f64")]
    price: f64,
    #[serde(deserialize_with = "parse_string_to_f64")]
    volume: f64,
    #[serde(deserialize_with = "parse_string_to_datetime")]
    time: DateTime<Utc>,
    side: String,
    order_type: String,
    misc: String,
    #[serde(skip_deserializing)]
    symbol_id: Option<i32>,
}

impl Trade {
    pub fn new(
        price: f64,
        volume: f64,
        time: DateTime<Utc>,
        side: String,
        order_type: String,
        misc: String,
        symbol_id: Option<i32>,
    ) -> Trade {
        Trade {
            price,
            volume,
            time,
            side,
            order_type,
            misc,
            symbol_id,
        }
    }

    // getter 
    pub fn get_time(&self) -> &DateTime<Utc> {
        &self.time
    } 

    pub fn get_price(&self) -> &f64 {
        &self.price
    }

    pub fn get_volume(&self) -> &f64 {
        &self.volume
    }

    pub fn get_side(&self) -> &str {
        &self.side[..]
    }

    pub fn get_order_type(&self) -> &str {
        &self.order_type[..]
    }

    pub fn get_symbol_id(&self) -> &Option<i32> {
        &self.symbol_id
    }

    // set for symbol_id
    pub fn set_symbol_id(&mut self, symbol_id: i32) {
        self.symbol_id = Some(symbol_id);
    }
}

impl From<&Trade> for TradeModel {
    fn from(value: &Trade) -> Self {
        TradeModel::new(
            value.get_time().to_owned(),
            value.get_price().to_owned(),
            value.get_volume().to_owned(),
            value.get_side().to_owned(),
            value.get_order_type().to_owned(),
            value.get_symbol_id().to_owned().unwrap(),
        )
    }
}

//  deserialize string to f64
fn parse_string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringToFloatVisitor;

    impl<'de> Visitor<'de> for StringToFloatVisitor {
        type Value = f64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representing a float")
        }

        fn visit_str<E>(self, value: &str) -> Result<f64, E>
        where
            E: de::Error,
        {
            value.parse::<f64>().map_err(E::custom)
        }
    }

    deserializer.deserialize_str(StringToFloatVisitor)
}


// deserialize string to DateTime<Utc>
fn parse_string_to_datetime<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringToDateTimeVisitor;

    impl<'de> Visitor<'de> for StringToDateTimeVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string representing a timestamp")
        }

        fn visit_str<E>(self, value: &str) -> Result<DateTime<Utc>, E>
        where
            E: de::Error,
        {
            let epoch_seconds: f64 = value.parse().map_err(E::custom)?;
            let datetime = Utc.timestamp_opt(
                epoch_seconds as i64,
                (epoch_seconds.fract() * 1_000_000.0) as u32,
            ).single().ok_or_else(|| E::custom("Invalid timestamp"))?;
            Ok(datetime)
        }
    }

    deserializer.deserialize_str(StringToDateTimeVisitor)
}



/*
use crate::utils::timestamp::parse_timestamp;
use chrono::{DateTime, Utc};
use repository::domain::trade::TradeModel;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, sqlx::FromRow, Deserialize, Debug, Clone)]
pub struct Trade {
    //pub channel_id: i32,
    time: DateTime<Utc>,
    price: f64,
    volume: f64,
    side: String,
    order_type: String,
    symbol: String,
    symbol_id: Option<i32>,
}

impl Trade {
    pub fn get_time(&self) -> &DateTime<Utc> {
        &self.time
    }
    pub fn get_price(&self) -> &f64 {
        &self.price
    }
    pub fn get_volume(&self) -> &f64 {
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
    pub fn get_symbol_id(&self) -> &Option<i32> {
        &self.symbol_id
    }

    // setter for symbol_id
    pub fn set_symbol_id(&mut self, symbol_id: i32) {
        self.symbol_id = Some(symbol_id);
    }
}

impl Trade {
    pub fn new(
        time: DateTime<Utc>,
        price: f64,
        volume: f64,
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
            symbol_id: None,
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
                        trade_array
                            .get(0)
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .parse::<f64>()
                            .unwrap(),
                        trade_array
                            .get(1)
                            .unwrap()
                            .as_str()
                            .unwrap()
                            .parse::<f64>()
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
            value.get_symbol_id().to_owned().unwrap(),
        )
    }
}
 */
