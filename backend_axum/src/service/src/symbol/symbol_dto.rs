use repository::domain::symbol::SymbolModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SymbolDto {
    symbol: String
}

impl SymbolDto {
    pub fn new(
        symbol: String
    ) -> Self {
        SymbolDto {
            symbol
        }
    }
}

impl From<SymbolModel> for SymbolDto {
    fn from(value: SymbolModel) -> Self {
        SymbolDto::new(
            value.get_symbol().to_owned()
        )
    }
}

impl From<SymbolDto> for SymbolModel {
    fn from(value: SymbolDto) -> Self {
        SymbolModel::new(
            value.symbol
        )
    }
}
