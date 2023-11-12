use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use repository::domain::trade::TradeModel;
use serde::{Deserialize, Serialize};

//TODO maybe use Optional for side, order_type as these values might not be avaliable

#[derive(Serialize, Deserialize)]
pub struct TradeDto {
    time: DateTime<Utc>,
    price: BigDecimal,
    volume: BigDecimal,
    side: String,
    order_type: String,
    symbol: String,
}

impl TradeDto {
    pub fn new(
        time: DateTime<Utc>,
        price: BigDecimal,
        volume: BigDecimal,
        side: String,
        order_type: String,
        symbol: String,
    ) -> Self {
        TradeDto {
            time,
            price,
            volume,
            side,
            order_type,
            symbol,
        }
    }
}

impl From<TradeModel> for TradeDto {
    fn from(value: TradeModel) -> Self {
        TradeDto::new(
            value.get_time().to_owned(),
            value.get_price().to_owned(),
            value.get_volume().to_owned(),
            value.get_side().to_owned(),
            value.get_order_type().to_owned(),
            value.get_symbol().to_owned(),
        )
    }
}

impl From<TradeDto> for TradeModel {
    fn from(value: TradeDto) -> Self {
        TradeModel::new(
            value.time,
            value.price,
            value.volume,
            value.side,
            value.order_type,
            value.symbol,
        )
    }
}
