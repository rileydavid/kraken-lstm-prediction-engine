use crate::model::trade::Trade;
use crate::repository::websocket::rt::task::JoinHandle;
use std::sync::{Arc, Mutex};
use serde_json::Value;
use actix_web::rt;
use log::info;
use tungstenite::{connect, Message};
use super::postgresdb::PostgresRepository;

// maybe think about combining Collector with postgres

pub struct Websocket {
    thread_handle: JoinHandle<Result<String, Box<dyn std::error::Error>>>,
    data: Arc<Mutex<Vec<Trade>>>, //not in use yet
}

impl Websocket {
    pub fn init(db: PostgresRepository) -> Websocket {
        let data = Arc::new(Mutex::new(Vec::new()));
        let handle = rt::spawn(Self::socket_loop(data.clone(), db));
        
        return Websocket {
            thread_handle: handle,
            data: data,
        };
    }

    pub async fn get_web_data(&self) -> Option<String> {
        info!("{:?}", self.data.lock().unwrap());
        return Some(String::from("Ok"));
    }

    pub async fn close_websocket(&self) -> Option<String> {
        // kinda rough
        self.thread_handle.abort();
        return Some(String::from("Ok"));
    }

    async fn socket_loop(
        _data: Arc<Mutex<Vec<Trade>>>, //for later 
        db: PostgresRepository,
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

            if cache.len() > 10 {
                let count = cache.len();
                for trade in cache.iter() {
                    let _ = db.insert_trade(trade).await; // error could happend whilst inserting (ignored at the moment)
                }
                //data.lock().unwrap().extend(cache.clone()); // Transfer the new trades to the shared data
                cache.clear();
                info!("Inserted {:?} Trades", count);
            }
        }
    }
}
