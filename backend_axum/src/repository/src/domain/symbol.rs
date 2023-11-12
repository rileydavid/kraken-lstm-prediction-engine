use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

//TODO: maybe an array would be better suited

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SymbolModel {
    symbol: String,
}

impl SymbolModel {
    pub fn new(
        symbol: String
    ) -> SymbolModel {
        SymbolModel {
            symbol
        }
    }
}

impl SymbolModel {
    pub fn get_symbol(&self) -> &str {
        &self.symbol[..]
    }
}

impl From<PgRow> for SymbolModel {
    fn from(value: PgRow) -> Self {
        SymbolModel::new(
            value.get("symbol"),
        )
    }
}
