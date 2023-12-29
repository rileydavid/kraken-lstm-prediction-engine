use cache::domain::symbol::SymbolCacheModel;
use repository::domain::symbol::SymbolModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SymbolDto {
    id: i32,
    symbol: String
}

impl SymbolDto {
    pub fn new(
        id: i32,
        symbol: String
    ) -> Self {
        SymbolDto {
            id,
            symbol
        }
    }
}

impl From<&SymbolModel> for SymbolDto {
    fn from(value: &SymbolModel) -> Self {
        SymbolDto::new(
            value.get_id().to_owned(),
            value.get_symbol().to_owned()
        )
    }
}

impl From<&SymbolCacheModel> for SymbolDto {
    fn from(value: &SymbolCacheModel) -> Self {
        SymbolDto::new(
            value.get_id().to_owned() as i32,
            value.get_symbol().to_owned()
        )
    }
}

impl From<SymbolDto> for SymbolModel {
    fn from(value: SymbolDto) -> Self {
        SymbolModel::new(
            value.id,
            value.symbol
        )
    }
}


