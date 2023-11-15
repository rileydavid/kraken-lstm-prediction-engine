use std::net::TcpStream;

use crate::dto::trade::Trade;
use repository::{domain::trade::TradeModel, repository::trade_repository};
use serde_json::Value;
use tracing::{error, info};
use tungstenite::{connect, stream::MaybeTlsStream, Message, WebSocket};
use utils::core::postgresdb::{Tx, TxAsync};

const CONST_ETHUSD_SUB: &str =
    "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"ETH/USD\"]}";

/*
const CONST_XBTUSD_SUB: &str =
    "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XBT/USD\"]}";
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

async fn websocket_loop(mut socket: WebSocket<MaybeTlsStream<TcpStream>>) {
    let mut collected_trades: Vec<Trade> = Vec::new(); // cache --> insert trades in batches

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

                    for trade in collected_trades.iter() {
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
                                error!("{:?}", err);
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
