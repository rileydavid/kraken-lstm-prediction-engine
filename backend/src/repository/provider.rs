use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use actix::prelude::*;
use actix_web::web::Data;
use actix_web_actors::ws;

use log::info;
use uuid::Uuid;

use super::cachemanager::CacheManager;

const UPDATE_INTERVAL: Duration = Duration::from_secs(2);
//const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
//const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Provider {
    hb: Instant,
    cache_manager: Data<CacheManager>,
    client: Option<Uuid>, /*
                          cache_trade: Cache<String, Trade>,
                          cache_trade_prediction: Cache<String, Trade>,
                          cache_serial: Cache<String, i16>
                           */
}

impl Provider {
    pub fn new(cache_manager: Data<CacheManager>) -> Self {
        Self {
            hb: Instant::now(),
            cache_manager: cache_manager,
            client: None,
        }
    }

    // This function will run on an interval, every 5 seconds to check
    // that the connection is still alive. If it's been more than
    // 10 seconds since the last ping, we'll close the connection.
    /*
        fn hb(&self, ctx: &mut <Self as Actor>::Context) {
            ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
                if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                    ctx.stop();
                    return;
                }

                ctx.ping(b"");
            });
        }


           let latest = act.cache_serial.get("trade").unwrap();

                let start = self.cache_serial.get("latest").unwrap();

                info!("Latest {:?}, Start {:?}", latest, start);

                for index in start..latest {
                    let temp = String::from("XBTUSD").add(&index.to_string());
                    info!("get string {:?}", &temp);
                    match self.cache_trade.get(&temp) {
                        Some(trade) => {
                            info!("Got some trade {:?}", trade.to_json().to_string());
                            ctx.text(trade.to_json().to_string())
                        }
                        None => {}
                    }
                }
    self.cache_serial.insert(String::from("latest"), latest);
         */

    pub fn updates(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(UPDATE_INTERVAL, |act, ctx| {
            info!("Run Interval Client {:?}", act.client);

            let uuid = act.client.unwrap().clone();

            let trades = act.cache_manager.get_trades_subscription(uuid);
            //.get_trades_subscription(uuid);
            info!("trades vec in update {:?}", trades);

            match trades.len() > 0 {
                true => {
                    for trade in trades {
                        info!("Got some trade {:?}", trade.to_json().to_string());
                        ctx.text(trade.to_json().to_string())
                    }
                }
                false => {
                    ctx.text(String::from("heartbeat"));
                }
            }
        });
    }
}

impl Actor for Provider {
    type Context = ws::WebsocketContext<Self>;

    // Start the heartbeat process for this connection
    fn started(&mut self, ctx: &mut Self::Context) {
        //self.hb(ctx);
        self.updates(ctx);
    }
}

// The `StreamHandler` trait is used to handle the messages that are sent over the socket.
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Provider {
    // The `handle()` function is where we'll determine the response
    // to the client's messages. So, for example, if we ping the client,
    // it should respond with a pong. These two messages are necessary
    // for the `hb()` function to maintain the connection status.
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            // Ping/Pong will be used to make sure the connection is still alive
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            // Text will echo any text received back to the client (for now)
            Ok(ws::Message::Text(text)) => {
                // needed some check if symbol is valid

                let uuid = Uuid::new_v4();
                let symbol = text.to_string();

                self.client = Some(uuid);

                match self.cache_manager.subscriptions.get(&symbol) {
                    Some(hashset) => {
                        let mut temp = HashSet::new();
                        for element in hashset.iter() {
                            temp.insert(element.to_owned());
                        }
                        temp.insert(uuid);
                        self.cache_manager
                            .subscriptions
                            .insert(symbol, temp.to_owned());
                    }
                    None => {
                        //info!("Addding Sub {}", uuid);
                        let mut hashset = HashSet::new();
                        hashset.insert(uuid);
                        self.cache_manager.subscriptions.insert(symbol, hashset);
                    }
                }

                self.cache_manager.client.insert(uuid, Vec::new());

                ctx.text(uuid.to_string());
            }
            // Close will close the socket
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}
