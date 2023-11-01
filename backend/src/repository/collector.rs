use crate::model::trade::Trade;

use actix_web::rt;
use log::{error, info};
use serde_json::Value;
use sqlx::PgPool;
use std::sync::{Arc,Mutex};
use tungstenite::{connect, Message};

use super::{postgresdb::PostgresRepository, cachemanager::CacheManager};

#[allow(dead_code)]
#[derive(Clone)]
pub struct Collector {
    thread_info: ThreadInfo,
    cache_manager: CacheManager,
    pool: Arc<PgPool>,
    db: PostgresRepository
}

// used to terminate websocket connections
#[derive(Clone)]
pub struct ThreadInfo {
    terminate_flag: Arc<Mutex<bool>>,
}

impl Collector {
    pub fn init(pool: Arc<PgPool>, cache_manager: CacheManager, db: PostgresRepository) -> Collector {
        let thread_info = ThreadInfo {
            terminate_flag: Arc::new(Mutex::new(false)),
        };

        // thread will run until the webserver crashes or the terminate flag is set
        let _handle = rt::spawn(Self::kraken_socket_loop(
            thread_info.clone(),
            cache_manager.clone(),
            db.clone()
        ));    

        return Collector {
            thread_info: thread_info,
            cache_manager: cache_manager,
            pool: pool.clone(),
            db  
        };
    }

    pub async fn close_websocket(&self) -> Option<String> {
        *self.thread_info.terminate_flag.lock().unwrap() = true;
        return Some(String::from("Ok"));
    }

    //TODO break this function up into smaller parts
    async fn kraken_socket_loop(
        thread_info: ThreadInfo,
        cache_manager: CacheManager,
        db: PostgresRepository
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
                Err(err) => error!("Error occured subscribing {:?}", err),
            };
        }

        let mut collected_trades: Vec<Trade> = Vec::new(); // cache --> insert trades in batches
   
        loop {
            if socket.can_read() {
                let msg = socket.read().unwrap().to_string();

                // only parse when it's not a heartbeat or subscription status
                if !msg.contains("heartbeat")
                    && !msg.contains("connectionID")
                    && !msg.contains("subscriptionStatus")
                {
                    let json: Value = serde_json::from_str(&msg).unwrap();
                    collected_trades.extend(Trade::parse(json).unwrap());

                    for trade in collected_trades.iter(){
                        match db.insert_trade(trade.to_owned()).await {
                            Ok(_) => {
                                info!("Inserted Trade");
                            },
                            Err(err) => {
                                error!("Error Occured during Insert {:?}",  err);
                            }
                        }
                    }

                    collected_trades.clear();
                }
            }
         
            if thread_info.terminate_flag.lock().unwrap().to_owned() {
                info!("Returning from thread");
                return None;
            }
        }
    }
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