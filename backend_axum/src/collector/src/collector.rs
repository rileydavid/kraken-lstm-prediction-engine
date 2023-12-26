use std::{net::TcpStream, collections::HashMap, ops::Deref};

use crate::dto::trade::Trade;
use repository::{domain::{trade::TradeModel, symbol::{self, SymbolModel}}, repository::{trade_repository, symbol_repository}};
use serde_json::Value;
use tracing::{error, info};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use utils::{core::postgresdb::{Tx, TxAsync}, error::generic_error::GenericError};

const CONST_ETHUSD_SUB: &str =
    "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"ETH/USD\"]}";


/* 
const CONST_XBTUSD_SUB: &str =
    "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XBT/USD\"]}";

const SUBSCRIPTIONS: [&str; 2] = [CONST_ETHUSD_SUB, CONST_XBTUSD_SUB];

    */

const SUBSCRIPTIONS: [&str; 1] = [CONST_ETHUSD_SUB];

pub async fn run() {
    info!("Starting Collector");

    let mut socket = connect("wss://ws.kraken.com").expect("Could not connect").0;

    socket = subscribe(socket);

    tokio::spawn(async move {
        websocket_loop(socket.into()).await;
    });
}

// todo clean up here and create more functions to make the code more readable!
async fn websocket_loop(mut socket: WebSocket<MaybeTlsStream<TcpStream>>) {
    let mut collected_trades: Vec<Trade> = Vec::new(); // cache --> insert trades in batches
    let mut symbols: HashMap<String, i32> = HashMap::new();

    // get symbols that are already in the database

    //create function for this
    let mut tx = Tx::begin().await;
    let symbols_result: Result<Vec<symbol::SymbolModel>, utils::error::generic_error::GenericError> = symbol_repository::get_symbols(&mut tx).await;
    Tx::commit(tx).await;

    match symbols_result {
        Ok(symbols_vec) => {
            for symbol_model in symbols_vec {
                symbols.insert(symbol_model.get_symbol().to_string(), symbol_model.get_id().deref().to_owned());
            }
        }
        Err(err) => {
            error!("Could not populate Symbol lookup table {:?}", err);
        }
    }
 
    loop {
        match socket.can_read() {
            true => {
                let msg = socket.read().unwrap().to_string();

                // only parse when it's not a heartbeat or subscription status
                if !msg.contains("heartbeat")
                    && !msg.contains("connectionID")
                    && !msg.contains("subscriptionStatus")
                {
                    let json: Value = serde_json::from_str(&msg).unwrap();
                    collected_trades.extend(Trade::parse_from_json(json).unwrap());

                    // check if symbol is in the hashmap
                    // if not, insert it into the database
                    // if it is there add it to the trade

                    
                    for trade in collected_trades.iter_mut() {
                                                
                        if symbols.contains_key(trade.get_symbol()) {
                            let id: &i32  = symbols.get(trade.get_symbol()).unwrap();
                            trade.set_symbol_id(id.to_owned());
                        
                        }else {
                            info!("Inserting new symbol into database");
                            let mut tx = Tx::begin().await;
                            let symbol_model_resutl: Result<SymbolModel, GenericError> = symbol_repository::insert_symbol(&mut tx, trade.get_symbol().to_string()).await;
                            Tx::commit(tx).await;

                            match symbol_model_resutl {
                                Ok(symbol_model) => {
                                    symbols.insert(symbol_model.get_symbol().to_string(), symbol_model.get_id().deref().to_owned());
                                    trade.set_symbol_id(symbol_model.get_id().to_owned());
                                }
                                Err(err) => {
                                    error!("Could not insert new symbol {:?}", err);
                                }
                            }
                        }   
                        

                        let mut tx = Tx::begin().await;

                        //TODO maybe this can be improved
                        match trade_repository::insert_trade(
                            &mut tx,
                            TradeModel::from(trade.to_owned()),
                        )
                        .await
                        {
                            Ok(_) => {
                                Tx::commit(tx).await;
                                info!("Inserted Trade");
                            }
                            Err(err) => {
                                error!("Could not insert trade {:?}", err);
                            }
                        }
                    }
                    collected_trades.clear();
                }
            }
            false => {}
        }
    }
}

fn subscribe(
    mut socket: WebSocket<MaybeTlsStream<TcpStream>>,
) -> WebSocket<MaybeTlsStream<TcpStream>> {
    for subscription in SUBSCRIPTIONS {
        match socket.send(Message::Text(subscription.into())) {
            Ok(it) => it,
            Err(err) => error!("Error occured subscribing {:?}", err),
        };
    }
    socket
}
