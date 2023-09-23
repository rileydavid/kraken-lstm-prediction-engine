use crate::model::trade::Trade;
use actix_web::{rt, web::Data};

use chrono::Utc;
use log::{error, info};
use serde_json::Value;
use sqlx::PgPool;
use std::{
    ops::Add,
    str::FromStr,
    sync::{Arc, Mutex},
};
use tokio::io::Interest;
use tungstenite::{connect, Message};

use moka::sync::Cache;
use std::time::Duration;

use super::{postgresdb::PostgresRepository, cachemanager::CacheManager};

// maybe think about combining Collector with postgres

#[derive(Clone)]
pub struct Collector {
    thread_info: ThreadInfo,
    cache_manager: CacheManager
    /*
    pub cache_trade: Cache<String, Trade>,
    pub cache_trade_prediction: Cache<String, Trade>,
    pub cache_serial: Cache<String, i16>
     */
}

// used to terminate websocket connections
#[derive(Clone)]
pub struct ThreadInfo {
    terminate_flag: Arc<Mutex<bool>>,
}

impl Collector {
    pub fn init(pool: PgPool, cache_manager: CacheManager) -> Collector {
        let thread_info = ThreadInfo {
            terminate_flag: Arc::new(Mutex::new(false)),
        };


        let cache_trade: Cache<String, Trade> = Cache::builder()
            .time_to_live(Duration::from_secs(60 * 60)) // Time to live (TTL): 60 minutes
            .build();

        let cache_trade_prediction: Cache<String, Trade> = Cache::builder()
            .time_to_live(Duration::from_secs(60 * 60)) // Time to live (TTL): 60 minutes
            .build();

        let cache_serial: Cache<String, i16> = Cache::builder().build();

        cache_serial.insert(String::from("trade"), 0);
        cache_serial.insert(String::from("prediction"), 0);
        cache_serial.insert(String::from("latest"), 0);


        // thread will run until the webserver crashes or the terminate flag is set
        let _handle = rt::spawn(Self::kraken_socket_loop(
            pool,
            thread_info.clone(),
            cache_manager.clone()
            /*
            cache_trade.clone(),
            cache_serial.clone(),
             */
        ));

        return Collector {
            thread_info: thread_info,
            cache_manager: cache_manager  //not sure how clone will affect all this
            /*
            cache_trade: cache_trade,
            cache_trade_prediction: cache_trade_prediction,
            cache_serial: cache_serial
             */
        };
    }

    /*

      pub async fn start_kraken_websocket(self) -> Option<String> {
          //info!("{:?}", self.data.lock().unwrap());

          let _handle = rt::spawn(Self::socket_loop(data.clone(), pool, thread_info.clone()));

          if handle_mut.is_none() == true {
              drop(handle_mut);
              let handle = rt::spawn(Self::socket_loop(self.data.clone(), self.pool.clone()));
              self.thread_handle = Arc::new(Mutex::new(Some(handle)));
          }


          return Some(String::from("Websocket started"));
      }

    */
/*
    pub async fn post_trade_prediction(
        &self,
        trade: Trade,
        db: Data<PostgresRepository>,
        cache: Data<Collector>
    ) -> Option<String> {
        let mut serial = cache.cache_serial.get("prediction").unwrap();

        // reset serial to avoid running into an overflow
        if serial > 30000 {
            serial = 0;
        } else {
            serial += 1;
        }
        self.cache_trade_prediction
            .insert(trade.symbol.clone() + &serial.to_string(), trade.clone());
        db.insert_trade_prediction(trade).await;

        cache.cache_serial.insert(String::from("prediction"), serial);

        return Some(String::from("Ok"));
    }
 */

    /*
    pub async fn get_cache(self, symbol: &str, interval: &str) -> Option<Vec<Trade>> {
        let interval_duration =
            Duration::from_secs(interval.to_string().parse::<u64>().unwrap() * 60);
        let trades = self.cache_manager.get_trades(symbol.to_string(), interval.to_string());       
        return trades;
    }
     */

    /*
     //TODO: Add interval
     pub async fn get_cached_trades_prediction(&self, symbol: &str, interval: &str) -> Option<Vec<Trade>> {
        let interval_duration =
            Duration::from_secs(interval.to_string().parse::<u64>().unwrap() * 60);
        let mut trades: Vec<Trade> = Vec::new();

        for key_value_pair in self.cache_trade_prediction.iter() {
            if key_value_pair.0.to_string().contains(&symbol)
                && key_value_pair.1.time > (Utc::now() - interval_duration)
            {
                trades.push(key_value_pair.1);
            }
        }
        return Some(trades);
    }
 */
    pub async fn close_websocket(&self) -> Option<String> {
        *self.thread_info.terminate_flag.lock().unwrap() = true;
        //*test = true;
        //let t = self.thread_info.terminate_flag.get_mut()
        return Some(String::from("Ok"));
    }

    /*
    async fn python_socket_loop(
        pool: PgPool,
        thread_info: ThreadInfo,
        data_pool: Cache<String, Trade>){}
    */

    //TODO break this function up into smaller parts
    async fn kraken_socket_loop(
        pool: PgPool,
        thread_info: ThreadInfo,
        cache_manager: CacheManager,
        /*
        cache_trade: Cache<String, Trade>,
        cache_serial: Cache<String, i16>
        */
    ) -> Option<()> {
        let mut socket = connect("wss://ws.kraken.com").expect("Could not connect").0;

        let subscriptions = vec![
            "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XRP/USD\"]}",
            "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"ETH/USD\"]}",
            "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XBT/USD\"]}",
        ];

        // subscribe
        for msg in subscriptions {
            match socket.send(Message::Text(msg.into())) {
                Ok(it) => it,
                Err(err) => {
                    error!("error whilst subscribing");
                    return None;
                }
            };
        }

        let mut local_cache: Vec<Trade> = Vec::new(); // cache --> insert trades in batches
         ;
   
        loop {
            if socket.can_read() {
                let msg = socket.read().unwrap().to_string();

                // only parse when it's not a heartbeat or subscription status
                if !msg.contains("heartbeat")
                    && !msg.contains("connectionID")
                    && !msg.contains("subscriptionStatus")
                {
                    let json: Value = serde_json::from_str(&msg).unwrap();
                    local_cache.extend(Trade::parse(json).unwrap());
                }
            }

            if local_cache.len() >= 5 { //figure out what a good value here would be
                let count = local_cache.len();
                for trade in local_cache.iter() {
                    //TODO create dedicated function for this and bulk insert --> less strain on the database
                    let result = sqlx::query("insert into kraken_trade (time, price, volume, side, order_type, symbol) values ($1, $2, $3, $4, $5, $6)")
                    .bind(&trade.time)
                    .bind(sqlx::types::BigDecimal::from_str(&trade.price.to_string()).unwrap())
                    .bind(sqlx::types::BigDecimal::from_str(&trade.volume.to_string()).unwrap())
                    .bind(&trade.side)
                    .bind(&trade.order_type)
                    .bind(&trade.symbol)
                    .execute(&pool).await;

                    if result.is_err() {
                        error!("Insert Failed");
                    }

                    cache_manager.add_trade(trade.to_owned());
                }
                //data.lock().unwrap().extend(cache.clone()); // Transfer the new trades to the shared data
                local_cache.clear();
                info!("Inserted {:?} Trades", count);

                // terminates the thread
                if thread_info.terminate_flag.lock().unwrap().to_owned() {
                    info!("Returning from thread");
                    return None;
                }
            }
        }
    }
}
