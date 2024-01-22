use serde::{Deserialize, Serialize};

use crate::dto::trade::Trade;

// struct for the entire message (from websocket)
#[derive(Serialize, Deserialize, Debug)]
pub struct TradeMessage {
    channel_id: i32,
    trades: Vec<Trade>,
    channel_name: String,
    symbol: String
}

impl TradeMessage {
    pub fn get_trades(&self) -> &Vec<Trade> {
        &self.trades
    }

    pub fn get_trades_mut(&mut self) -> &mut Vec<Trade> {
        &mut self.trades
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol[..]
    }
}
