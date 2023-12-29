use std::{collections::HashMap, net::TcpStream};

use crate::dto::trade::Trade;
use repository::{
    domain::{
        symbol::{self, SymbolModel},
        trade::TradeModel,
    },
    repository::{symbol_repository, trade_repository},
};
use serde_json::Value;
use sqlx::PgPool;
use tracing::{error, info};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use utils::{error::generic_error::GenericError, core::postgresdb};

use rayon::prelude::*;

const CONST_ETHUSD_SUB: &str =
    "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"ETH/USD\"]}";

const SUBSCRIPTIONS: [&str; 1] = [CONST_ETHUSD_SUB];

/*
const CONST_XRPUSD_SUB: &str =
    "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XRP/USD\"]}";

const CONST_XBTUSD_SUB: &str =
    "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XBT/USD\"]}";
const SUBSCRIPTIONS: [&str; 3] = [CONST_ETHUSD_SUB, CONST_XBTUSD_SUB, CONST_XRPUSD_SUB];
 */

pub async fn run() {
    info!("Starting Collector");

    let mut socket = connect("wss://ws.kraken.com").expect("Could not connect").0;

    // subscribe to default symbols
    socket = subscribe(socket).await;

    tokio::spawn(async move {
        websocket_loop(socket.into()).await;
    });
}

async fn websocket_loop(mut socket: WebSocket<MaybeTlsStream<TcpStream>>) {
    let pool: &sqlx::PgPool = postgresdb::get_connection().await;
    let mut symbols: HashMap<String, i32> = init_symbols(pool).await;
    
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
                    let mut collected_trades = Trade::parse_from_json(json).unwrap();

                    for trade in collected_trades.iter_mut() {
                        if symbols.contains_key(trade.get_symbol()) {
                            trade.set_symbol_id(*symbols.get(trade.get_symbol()).unwrap());
                        } else {
                            let _ =
                                insert_new_symbol(pool,&mut symbols, trade, trade.get_symbol().into())
                                    .await;
                        }
                    }

                    let _ = insert_trades(pool, collected_trades).await;
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

async fn insert_new_symbol(pool: &PgPool, symbols: &mut HashMap<String, i32>, trade: &mut Trade, symbol: String) {
    info!("Inserting new symbol into database");

    let symbol_model_resutl: Result<SymbolModel, GenericError> =
        symbol_repository::insert_symbol(pool, symbol).await;

    match symbol_model_resutl {
        Ok(symbol_model) => {
            symbols.insert(
                symbol_model.get_symbol().to_string(),
                symbol_model.get_id().to_owned(),
            );
            trade.set_symbol_id(symbol_model.get_id().to_owned());
        }
        Err(err) => {
            error!("Could not insert new symbol {:?}", err);
        }
    }
}

async fn insert_trades(pool: &PgPool, trades: Vec<Trade>) {
    let count = trades.len();

    let trades = trades
        .into_par_iter()
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
) -> WebSocket<MaybeTlsStream<TcpStream>> {
    for subscription in SUBSCRIPTIONS {
        match socket.send(Message::Text(subscription.into())) {
            Ok(it) => it,
            Err(err) => error!("Error occured subscribing {:?}", err),
        };
    }
    socket
}
