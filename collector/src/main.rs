use std::fmt;
use std::str::FromStr;

use anyhow::Error;
use chrono::NaiveDateTime;
use serde_json::Value;

use sqlx::{postgres::PgPoolOptions, types::BigDecimal, PgPool};
use tungstenite::{
    connect, handshake::client::Response, stream::MaybeTlsStream, Message, WebSocket,
};

#[allow(dead_code)]
struct Trade {
    channel_id: i32,
    time: NaiveDateTime,
    price: BigDecimal,
    volume: BigDecimal,
    side: String,
    order_type: String,
    symbol: String,
}

impl Trade {
    async fn parse(json: Value) -> Result<Vec<Trade>, Error> {
        let json_value = json.as_array().unwrap();
        let channel_id = json_value
            .get(0)
            .unwrap()
            .as_number()
            .unwrap()
            .to_string()
            .parse::<i32>()
            .unwrap();
        let symbol = json_value.get(3).unwrap().as_str().unwrap().replace("/", "");

        let mut trades = Vec::new();

        //skipping channel id
        for value in json_value.iter().skip(1) {
            if value.is_array() {
                for array in value.as_array().unwrap() {
                    let trade_array = array.as_array().unwrap();

                    trades.push(Trade::new(
                        channel_id,
                        trade_array.get(2).unwrap().as_str().unwrap(),
                        trade_array.get(0).unwrap().as_str().unwrap(),
                        trade_array.get(1).unwrap().as_str().unwrap(),
                        trade_array.get(3).unwrap().as_str().unwrap(),
                        trade_array.get(4).unwrap().as_str().unwrap(),
                        &symbol,
                    ));
                }
            }
        }
        return Ok(trades);
    }

    fn new(
        channel_id: i32,
        time: &str,
        price: &str,
        volume: &str,
        side: &str,
        order_type: &str,
        symbol: &str,
    ) -> Trade {
        Trade {
            channel_id: channel_id,
            time: NaiveDateTime::from_timestamp_micros(
                (time.parse::<f64>().unwrap() * 1000000.0) as i64,
            )
            .unwrap(),
            price: BigDecimal::from_str(price).unwrap(),
            volume: BigDecimal::from_str(volume).unwrap(),
            side: side.to_string(),
            order_type: order_type.to_string(),
            symbol: symbol.to_string(),
        }
    }
}

impl fmt::Display for Trade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(Channel_ID={}, Time={}, Price={}, Volume={}, Side={}, Order_Type={}, Symbol={})",
            self.channel_id,
            self.time,
            self.price,
            self.volume,
            self.side,
            self.order_type,
            self.symbol
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("Starting Collector!");

    let pool = get_pool().await;

    let mut socket = get_socket().0;

    let subscriptions = vec![
        "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XRP/USD\"]}",
        /*
        "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"ETH/USD\"]}",
        */
        "{\"event\":\"subscribe\", \"subscription\":{\"name\":\"trade\"}, \"pair\":[\"XBT/USD\"]}",
    ];

    // subscribe
    for msg in subscriptions {
        socket.send(Message::Text(msg.into()))?;
    }

    // get pool ready
    let pool = pool.unwrap();

    loop {
        let msg = socket.read().unwrap().to_string();

        if !msg.contains("heartbeat") && !msg.contains("connectionID") {
            let json: Value = serde_json::from_str(&msg).unwrap();

            if json.is_array() {
                let trades = Trade::parse(json).await;
                if trades.is_ok() {
                    let vec = trades.unwrap();
                    for trade in vec {
                        let _ = insert_trade(trade, &pool).await;
                    }
                } else {
                    println!("Error occured whilst parsing Trade");
                }
            }
        }
    }
}

async fn get_pool() -> Result<PgPool, sqlx::Error> {
    //172.1.0.10
    return PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://admin:password@172.1.0.10:5432/db")
        .await;
}

fn get_socket() -> (WebSocket<MaybeTlsStream<std::net::TcpStream>>, Response) {
    return connect("wss://ws.kraken.com").expect("Can't connect");
}

async fn insert_trade(trade: Trade, pool: &PgPool) -> Result<(), Error> {
    match sqlx::query("insert into kraken_trade (time, price, volume, side, order_type, symbol) values ($1, $2, $3, $4, $5, $6)")
    .bind(trade.time)
    .bind(trade.price)
    .bind(trade.volume)
    .bind(trade.side)
    .bind(trade.order_type)
    .bind(trade.symbol)
    .execute(pool)
    .await
    {
        Ok(_) => {
            println!("Inserted Trade: {:?}", trade.time);
            return Ok(());
        }
        Err(_) => {
            println!("Error whilst inserting trade");
            return Err(Error::msg("Trade could not be inserted"));
        }
    };
}
