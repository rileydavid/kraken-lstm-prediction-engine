use crate::symbol::symbol_dto::SymbolDto;
use repository::repository::symbol_repository;
use utils::core::postgresdb::Tx;
use utils::core::postgresdb::TxAsync;
use utils::error::generic_error::GenericError;

pub async fn get_symbols() -> Result<Vec<SymbolDto>, GenericError> {

    let mut tx = Tx::begin().await;
    match symbol_repository::get_symbols(&mut tx).await {
        Ok(data) => {
            Tx::commit(tx).await;
            let result: Vec<SymbolDto> = data
                .into_iter()
                .map(|model| SymbolDto::from(model))
                .collect();
            Ok(result)
        }
        Err(err) => Err(err),
    }
}