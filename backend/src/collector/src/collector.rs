use crate::dto::trade::Trade;
use crate::dto::trade_message::TradeMessage;
use repository::{
    domain::{
        symbol::{self, SymbolModel},
        trade::TradeModel,
    },
    repository::{symbol_repository, trade_repository},
};
use std::{collections::HashMap, net::TcpStream};

use sqlx::PgPool;
use tracing::{error, info};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use utils::{core::postgresdb, error::generic_error::GenericError};

use rayon::prelude::*;

use tokio::sync::mpsc::Receiver;

/*
const CONST_ETHUSD_SUB: &str =
    "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"ETH/USD\"]}";

const SUBSCRIPTIONS: [&str; 1] = [CONST_ETHUSD_SUB];
*/

pub async fn run(receiver: Receiver<String>) {
    info!("Starting Collector");
    tokio::spawn(async move {
        websocket_loop(receiver).await;
    });
}

async fn websocket_loop(
    mut receiver: Receiver<String>,
) {
    let mut socket = connect("wss://ws.kraken.com").expect("Could not connect").0;
    let pool: &sqlx::PgPool = postgresdb::get_connection().await;
    let mut symbols: HashMap<String, i32> = init_symbols(pool).await;

    // subscribe to symbols
    socket = subscribe(socket, symbols.keys().cloned().collect()).await;

    loop {
        let sub_msg = receiver.try_recv();

        if sub_msg.is_ok() {
            socket = subscribe(socket, vec![sub_msg.unwrap().into()]).await;
        }

        match socket.can_read() {
            true => {
                let msg = socket.read().unwrap().to_string();

                // only parse when it's not a heartbeat or subscription status
                if !msg.contains("heartbeat")
                    && !msg.contains("connectionID")
                    && !msg.contains("subscriptionStatus")
                {
                    // trade message
                    let mut trade_message: TradeMessage = serde_json::from_str(&msg).unwrap();

                    // insert symbol id
                    if !symbols.contains_key(trade_message.get_symbol()) {
                        let _ = insert_new_symbol(
                            pool,
                            &mut symbols,
                            trade_message.get_symbol().into(),
                        )
                        .await;
                    }

                    let symbol_id = symbols.get(trade_message.get_symbol()).unwrap().to_owned();

                    for trade in trade_message.get_trades_mut().iter_mut() {
                        trade.set_symbol_id(symbol_id);
                    }

                    let _ = insert_trades(pool, trade_message.get_trades()).await;
                } else {
                    // TODO
                    // if new sub is added check if it was received properly
                    // check if heartbeat intervall is still good
                    // info!(msg);
                    //thread::sleep(std::time::Duration::from_millis(100));
                }
            }
            false => {}
        }
    }
}

async fn init_symbols(pool: &PgPool) -> HashMap<String, i32> {
    let mut symbols: HashMap<String, i32> = HashMap::new();

    let symbols_result: Result<
        Vec<symbol::SymbolModel>,
        utils::error::generic_error::GenericError,
    > = symbol_repository::get_symbols(pool).await;

    match symbols_result {
        Ok(symbols_vec) => {
            for symbol_model in symbols_vec {
                symbols.insert(
                    symbol_model.get_symbol().to_string(),
                    symbol_model.get_id().to_owned(),
                );
            }
            symbols
        }
        Err(err) => {
            error!("Could not populate Symbol lookup table {:?}", err);
            HashMap::new()
        }
    }
}

async fn insert_new_symbol(pool: &PgPool, symbols: &mut HashMap<String, i32>, symbol: String) {
    info!("Inserting new symbol into database");

    let symbol_model_result: Result<SymbolModel, GenericError> =
        symbol_repository::insert_symbol(pool, symbol).await;

    match symbol_model_result {
        Ok(symbol_model) => {
            symbols.insert(
                symbol_model.get_symbol().to_string(),
                symbol_model.get_id().to_owned(),
            );
        }
        Err(err) => {
            error!("Could not insert new symbol {:?}", err);
        }
    }
}

async fn insert_trades(pool: &PgPool, trades: &Vec<Trade>) {
    let count = trades.len();

    let trades = trades
        .par_iter()
        .map(|trade| TradeModel::from(trade))
        .collect();

    match trade_repository::insert_trades(pool, trades).await {
        Ok(_) => {
            info!("Inserted {} Trade/s", count);
        }
        Err(err) => {
            error!("Could not insert trade {:?}", err);
        }
    }
}

async fn subscribe(
    mut socket: WebSocket<MaybeTlsStream<TcpStream>>,
    sub_symbols: Vec<String>,
) -> WebSocket<MaybeTlsStream<TcpStream>> {
    for subscription in sub_symbols {
        // could be it's own struct
        let sub_msg = format!(
            "{{\"event\":\"subscribe\", \"subscription\":{{\"name\":\"trade\"}}, \"pair\":[\"{}\"]}}",
            subscription
        );

        match socket.send(Message::Text(sub_msg.into())) {
            Ok(it) => it,
            Err(err) => error!("Error occured subscribing {:?}", err),
        };
    }
    socket
}
