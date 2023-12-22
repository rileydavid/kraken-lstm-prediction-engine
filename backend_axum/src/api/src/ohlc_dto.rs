use serde::{Deserialize, Serialize};
use repository::domain::ohlc::OhlcModel;

#[derive(Serialize, Deserialize)]
pub struct OhlcDto {
    id: i32,
    symbol: String
}

impl OhlcDto {
    pub fn new(
        id: i32,
        symbol: String
    ) -> Self {
        OhlcDto {
            id,
            symbol
        }
    }
}

impl From<OhlcModel> for OhlcDto {
    fn from(value: SymbolModel) -> Self {
        SymbolDto::new(
            value.get_id().to_owned(),
            value.get_symbol().to_owned()
        )
    }
}

impl From<OhlcDto> for OhlcModel {
    fn from(value: SymbolDto) -> Self {
        SymbolModel::new(
            value.id,
            value.symbol
        )
    }
}
