use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Duration;

use bigdecimal::ToPrimitive;
use chrono::NaiveDateTime;
use chrono::Utc;
use linfa::prelude::*;
use linfa_linear::LinearRegression;
use log::info;
use moka::sync::Cache;
use ndarray::prelude::*;
use uuid;
use uuid::Uuid;

use crate::model::trade::Trade;

//TODO: Sadly the websocket does not get the correct updates figure out how this could be resolved --> maybe because
// it is using the data<CacheManager>

#[derive(Clone)]
pub struct CacheManager {
    trade: Cache<String, Trade>, // live trading data cache, using a serial number which counts up
    prediction: Cache<String, Trade>,
    pub client: Cache<Uuid, Vec<Trade>>, // think about this a little
    //client_subscriptions: HashMap<Uuid, HashSet<String>>,
    pub subscriptions: Cache<String, HashSet<Uuid>>, // not filled yet
}

impl CacheManager {
    pub fn new() -> CacheManager {
        return CacheManager {
            trade: Cache::builder()
                .time_to_live(Duration::from_secs(60 * 60))
                .build(), // TTL 60 mins
            prediction: Cache::builder()
                .time_to_live(Duration::from_secs(60 * 60))
                .build(), // TTL 60 mins
            client: Cache::builder()
                .time_to_live(Duration::from_secs(60 * 60))
                .build(), // TTL 60 mins
            //client_subscriptions: HashMap::new(),
            subscriptions: Cache::builder()
                .time_to_live(Duration::from_secs(60 * 60))
                .build(),
        };
    }

    pub fn get_trades(&self, symbol: String, interval: String) -> Option<Vec<Trade>> {
        let interval_duration =
            Duration::from_secs(interval.to_string().parse::<u64>().unwrap() * 60);

        let mut trades: Vec<Trade> = Vec::new();

        for key_value_pair in self.trade.iter() {
            match key_value_pair.0.to_string().contains(&symbol)
                && key_value_pair.1.time > (Utc::now() - interval_duration)
            {
                true => trades.push(key_value_pair.1),
                false => {}
            }
        }

        match trades.len() > 0 {
            true => return Some(trades),
            false => return None,
        }
    }

    pub fn get_prediction(&self, symbol: String, interval: String) -> Option<String> {
        let interval_duration =
            Duration::from_secs(interval.to_string().parse::<u64>().unwrap() * 60).as_millis()
                as f64;

        //info!("interval duration {:?}", interval_duration);

        let mut xs: Vec<f64> = Vec::new();
        let mut ys: Vec<f64> = Vec::new();

        let mut latest = 0.0;
        let mut price: f64 = 0.0;

        for trade in self.get_trades(symbol, interval).unwrap() {
            xs.push(trade.time.timestamp_millis() as f64);

            if latest < (trade.time.timestamp_millis() as f64) {
                latest = trade.time.timestamp_millis() as f64;
                price = trade.price.to_f64().unwrap();
            }

            ys.push(trade.price.to_f64().unwrap());
        }

        let shape = (xs.len(), 1);
        let array2 = Array2::from_shape_vec(shape, xs).unwrap();

        let shape = (1, ys.len());
        let array1 = Array1::from(ys);

        //info!("Array2 {:?}", array2);
        //info!("Array {:?}", array1);

        let dataset = Dataset::new(array2, array1);
        let model = LinearRegression::default().fit(&dataset).unwrap();

        let mut y_values: Array1<f64> = array![price];
        let dataset = Dataset::new(arr2(&[[latest + interval_duration]]), y_values);

        let temp: i64 = (latest + 900000.0) as i64;
        //info!("time {:?}", NaiveDateTime::from_timestamp_millis((latest+interval_duration) as i64));

        let prediction = model.predict(&dataset);

        //info!("Pred {:?}", prediction);

        return Some(prediction.to_string());

        /*
        for key_value_pair in self.prediction.iter() {
            match key_value_pair.0.to_string().contains(&symbol) && key_value_pair.1.time > (Utc::now() - interval_duration) {
                true => prediction.push(key_value_pair.1),
                false => {}
            }
        }

        match prediction.len() > 0 {
            true => return Some(prediction),
            false => return None
        }
         */
    }

    pub fn add_trade(&self, trade: Trade) {
        //info!("add trade called {:?}", &trade.symbol);
        match self.subscriptions.contains_key(&trade.symbol) {
            true => {
                //info!("subscriptions has matching symbol entry");

                for client in self.subscriptions.get(&trade.symbol).unwrap().iter() {
                    //info!("Client uuid? {:?}", client);
                    match self.client.get(client) {
                        Some(trades) => {
                            let mut temp: Vec<Trade> = Vec::new();
                            temp.extend(trades);
                            temp.push(trade.clone());
                            self.client.insert(client.to_owned(), temp);
                        }
                        None => {}
                    }
                }
            }
            false => {}
        }

        self.trade.insert(
            trade.symbol.clone() + "_" + &Uuid::new_v4().to_string(),
            trade,
        );
    }

    pub fn add_prediction(self, trade: Trade) {
        //TODO missing logic for prediction cache
        self.prediction.insert(
            trade.symbol.clone() + "_" + &Uuid::new_v4().to_string(),
            trade,
        );
    }

    pub fn add_client_subscription(self, symbol: String) -> Self {
        let uuid = Uuid::new_v4();

        match self.subscriptions.get(&symbol) {
            Some(hashset) => {
                let mut temp = HashSet::new();
                for element in hashset.iter() {
                    temp.insert(element.to_owned());
                }
                temp.insert(uuid);
                self.subscriptions.insert(symbol, temp.to_owned());
            }
            None => {
                //info!("Addding Sub {}", uuid);
                let mut hashset = HashSet::new();
                hashset.insert(uuid);
                self.subscriptions.insert(symbol, hashset);
            }
        };

        return self;

        /*
        match self.client_subscriptions.get(&uuid) {
            Some(hashset) => {
                let mut temp = HashSet::new();
                for element in hashset.iter(){
                    temp.insert(element.to_owned());
                }
                temp.insert(symbol);
                self.client_subscriptions.insert(uuid, hashset.to_owned());
            },
            None => {
                let mut hashset = HashSet::new();
                hashset.insert(symbol);
                self.client_subscriptions.insert(uuid, hashset);
            },
        }
         */
    }

    pub fn get_trades_subscription(&self, uuid: Uuid) -> Vec<Trade> {
        //nothing in there
        /*
        for element in self.client.iter() {
            info!("iter client {:?}", element.0);
        }
         */

        match self.client.get(&uuid) {
            Some(trades) => {
                self.client.insert(uuid.to_owned(), Vec::new());
                return trades.to_owned();
            }
            None => {
                info!("Entries for client none ");
                return Vec::new();
            }
        }
    }
}
