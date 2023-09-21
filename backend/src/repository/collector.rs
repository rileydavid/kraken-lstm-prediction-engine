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

// maybe think about combining Collector with postgres

#[derive(Clone)]
pub struct Collector {
    thread_info: ThreadInfo
    //data: Arc<Mutex<Vec<Trade>>>
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

        // thread will run until the webserver crashes or the terminate flag is set
        let _handle = rt::spawn(Self::socket_loop(data.clone(), pool, thread_info.clone()));
    
        return Collector{
            thread_info: thread_info
        }
    }
    
    // maybe use this to process insertions
    //pub fn test(db: Data<PostgresRepository>){}
    
    
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
    

    pub async fn close_websocket(&self) -> Option<String> {
        // kinda rough
        *self.thread_info.terminate_flag.lock().unwrap() = true;
        //*test = true;
        //let t = self.thread_info.terminate_flag.get_mut()
        return Some(String::from("Ok"));
    }



    //TODO break this function up into smaller parts
    async fn socket_loop(
        _data: Arc<Mutex<Vec<Trade>>>, //for later
        pool: PgPool,
        thread_info: ThreadInfo
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
                    
                    if result.is_err() {
                        error!("Insert Failed");

                    }
                }
                //data.lock().unwrap().extend(cache.clone()); // Transfer the new trades to the shared data
                cache.clear();
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
