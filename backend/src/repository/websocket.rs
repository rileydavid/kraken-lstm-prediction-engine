use super::postgresdb::PostgresRepository;
use crate::model::trade::Trade;
use crate::repository::websocket::rt::task::JoinHandle;
use actix_web::rt;
use chrono::{DateTime, Utc};
use log::{error, info};
use serde_json::Value;
use sqlx::{Acquire, PgPool};
use std::{
    str::FromStr,
    sync::{Arc, Mutex}, result,
};
use tungstenite::{connect, Message};

// maybe think about combining Collector with postgres


#[derive(Clone)]
pub struct Websocket {
    //thread_handle: Arc<Mutex<Option<JoinHandle<Result<String, Box<dyn std::error::Error>>>>>>,
    //pool: PgPool,
    data: Arc<Mutex<Vec<Trade>>>, //not in use yet
}

impl Websocket {
    pub fn init(pool: PgPool) -> Websocket {
        let data = Arc::new(Mutex::new(Vec::new()));
        let _handle = rt::spawn(Self::socket_loop(data.clone(), pool));

        return Websocket {
            //thread_handle: Arc::new(Mutex::new(None)),
            data: data,
        };
    }

    /*
    pub async fn start_kraken_websocket(self) -> Option<String> {
        //info!("{:?}", self.data.lock().unwrap());

        let handle_mut = self.thread_handle.lock().unwrap().as_mut();

        if handle_mut.is_none() == true {
            drop(handle_mut);
            let handle = rt::spawn(Self::socket_loop(self.data.clone(), self.pool.clone()));
            self.thread_handle = Arc::new(Mutex::new(Some(handle)));
        }
        
        
        return Some(String::from("Websocket started"));
    }

    
    pub async fn close_websocket(self) -> Option<String> {
        // kinda rough
        if self.thread_handle.is_some() {
            self.thread_handle?.abort();
        }
        return Some(String::from("Ok"));
    }
    
    
 */
    /*
    pub fn insert_trade(trade: &Trade, pool: PgPool) {
        // maybe there is a better way to convert the bigdecimal
        while pool.num_idle() != 0 {
            let _ = 
        }
    }
     */

    async fn socket_loop(
        _data: Arc<Mutex<Vec<Trade>>>, //for later
        pool: PgPool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut socket = connect("wss://ws.kraken.com").expect("Could not connect").0;

        let subscriptions = vec![
            "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XRP/USD\"]}",
            "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"ETH/USD\"]}",
            "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XBT/USD\"]}",
        ];

        // subscribe
        for msg in subscriptions {
            socket.send(Message::Text(msg.into()))?;
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
            }
        }
    }
}
