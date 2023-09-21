use crate::model::trade::Trade;
use actix_web::rt;

use log::{error, info};
use serde_json::Value;
use sqlx::PgPool;
use std::{
    str::FromStr,
    sync::{Arc, Mutex}
};
use tungstenite::{connect, Message};

use moka::sync::Cache;
use std::time::Duration;

// maybe think about combining Collector with postgres

#[derive(Clone)]
pub struct Collector {
    thread_info: ThreadInfo,
    data_pool: Cache<String, Trade>
}

// used to terminate websocket connections
#[derive(Clone)]
pub struct ThreadInfo {
    terminate_flag: Arc<Mutex<bool>>,
}

impl Collector {
    pub fn init(pool: PgPool) -> Collector {
        let data = Arc::new(Mutex::new(Vec::new()));

        let thread_info = ThreadInfo {
            terminate_flag: Arc::new(Mutex::new(false))
        };

        let cache: Cache<String, Trade> = Cache::builder()
        // Time to live (TTL): 60 minutes
        .time_to_live(Duration::from_secs(60 * 60))
        .build();


       // thread will run until the webserver crashes or the terminate flag is set
        let _handle = rt::spawn(Self::kraken_socket_loop(data.clone(), pool, thread_info.clone(), cache.clone()));
    
        return Collector{
            thread_info: thread_info,
            data_pool: cache
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
    
    //TODO: Add interval
    pub async fn get_cache(&self, symbol: &str) -> Option<Vec<Trade>> {
        let mut trades: Vec<Trade> = Vec::new();

        for key_value_pair in self.data_pool.iter() {
            if key_value_pair.0.to_string().contains(&symbol){
                trades.push(key_value_pair.1);
            }
        }
        return Some(trades);
    }



    pub async fn close_websocket(&self) -> Option<String> {
        *self.thread_info.terminate_flag.lock().unwrap() = true;
        //*test = true;
        //let t = self.thread_info.terminate_flag.get_mut()
        return Some(String::from("Ok"));
    }


    //TODO break this function up into smaller parts
    async fn kraken_socket_loop(
        _data: Arc<Mutex<Vec<Trade>>>, //for later
        pool: PgPool,
        thread_info: ThreadInfo,
        data_pool: Cache<String, Trade>
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
                    return None
                },
            };
        }

        let mut cache: Vec<Trade> = Vec::new(); // cache --> insert trades in batches
        let mut serial: i16 = 0;

        loop {
            if socket.can_read() {
                let msg = socket.read().unwrap().to_string();

                // only parse when it's not a heartbeat or subscription status
                if !msg.contains("heartbeat")
                    && !msg.contains("connectionID")
                    && !msg.contains("subscriptionStatus")
                {
                    let json: Value = serde_json::from_str(&msg).unwrap();
                    cache.extend(Trade::parse(json).unwrap());
                }
            }

            if cache.len() >= 10 {
                let count = cache.len();
                for trade in cache.iter() {
                    let result = sqlx::query("insert into kraken_trade (time, price, volume, side, order_type, symbol) values ($1, $2, $3, $4, $5, $6)")
                    .bind(&trade.time)
                    .bind(sqlx::types::BigDecimal::from_str(&trade.price.to_string()).unwrap())
                    .bind(sqlx::types::BigDecimal::from_str(&trade.volume.to_string()).unwrap())
                    .bind(&trade.side)
                    .bind(&trade.order_type)
                    .bind(&trade.symbol)
                    .execute(&pool).await;
                    
                    
                    data_pool.insert(trade.symbol.to_owned() + &serial.to_string(), trade.to_owned());
                    serial += 1;

                    if result.is_err() {
                        error!("Insert Failed");
                    }
                }
                //data.lock().unwrap().extend(cache.clone()); // Transfer the new trades to the shared data
                cache.clear();
                info!("Inserted {:?} Trades", count);

                //reset serial - this would allow a maximum of 500 data entries per minute
                if serial > 30000 { 
                    serial = 0;
                }

                // terminates the thread
                if thread_info.terminate_flag.lock().unwrap().to_owned() {
                    info!("Returning from thread");
                    return None;
                }
            }           
        }
    }
}
