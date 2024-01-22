use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SymbolCacheModel {
    id: i32,
    symbol: String,
}

impl SymbolCacheModel {
    pub fn new(id: i32, symbol: String) -> Self {
        SymbolCacheModel {
            id,
            symbol,
        }
    }

    pub fn get_id(&self) -> &i32 {
        &self.id
    }

    pub fn get_symbol(&self) -> &String {
        &self.symbol
    }
}


