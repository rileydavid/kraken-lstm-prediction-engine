use crate::trade::trade_dto::TradeDto;
use repository::repository::trade_repository;
use utils::core::postgresdb::Tx;
use utils::core::postgresdb::TxAsync;
use utils::error::generic_error::GenericError;

pub async fn get_trades_by_symbol_interval(
    symbol: String,
    interval: i32,
) -> Result<Vec<TradeDto>, GenericError> {
    let mut tx = Tx::begin().await;
    match trade_repository::get_trades_by_symbol_interval(&mut tx, symbol, interval).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<TradeDto> = data
                .into_iter()
                .map(|model| TradeDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}
