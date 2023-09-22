use std::{time::{Duration, Instant}, sync::{Arc, Mutex}, ops::Add};

use actix::prelude::*;
use actix_web::web::{Bytes, Data};
use actix_web_actors::ws;

use actix_web::{HttpRequest, web, HttpResponse, Error};
use log::info;
use moka::sync::Cache;

use crate::model::trade::Trade;

use super::collector::Collector;

const UPDATE_INTERVAL: Duration = Duration::from_millis(1000);
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct Provider {
    hb: Instant,
    cache_trade: Cache<String, Trade>,
    cache_trade_prediction: Cache<String, Trade>,
    cache_serial: Cache<String, i16>
}

impl Provider {
    pub fn new(cache: Data<Collector>) -> Self {
        Self { hb: Instant::now(), cache_trade: cache.cache_trade.clone(), cache_trade_prediction: cache.cache_trade_prediction.clone(), cache_serial:cache.cache_serial.clone()}
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
            let latest = act.cache_serial.get("trade").unwrap();
            let start = act.cache_serial.get("latest").unwrap();

            info!("Latest {:?}, Start {:?}", latest, start);

            for index in start..latest {
                let temp = String::from("XBTUSD").add(&index.to_string());
                info!("get string {:?}", &temp);
                match act.cache_trade.get(&temp) {
                    Some(trade) => {
                        info!("Got some trade {:?}", trade.to_json().to_string());
                        ctx.text(trade.to_json().to_string())
                    }
                    None => {}
                }
            }


            if latest == start {
                ctx.text(String::from("heartbeat"));
            }
        
            act.cache_serial.insert(String::from("latest"), latest);
            
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
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context, ) {

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
                ctx.text("Ok");

                // should receive predictions here

            },
            // Close will close the socket
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}