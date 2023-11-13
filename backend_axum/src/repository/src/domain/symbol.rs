use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

//TODO: maybe an array would be better suited

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SymbolModel {
    id: i32,
    symbol: String,
}

impl SymbolModel {
    pub fn new(
        id: i32,
        symbol: String
    ) -> SymbolModel {
        SymbolModel {
            id,
            symbol
        }
    }
}

impl SymbolModel {
    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_symbol(&self) -> &str {
        &self.symbol[..]
    }
}

impl From<PgRow> for SymbolModel {
    fn from(value: PgRow) -> Self {
        SymbolModel::new(
            value.get("out_id"),
            value.get("out_symbol"),
        )
    }
}
