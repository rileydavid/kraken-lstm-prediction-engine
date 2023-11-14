use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConvertModel {
    file_name: String,
    symbol: String
}

impl ConvertModel {
    pub fn new(
        file_name: String,
        symbol: String,
    ) -> ConvertModel {
        ConvertModel {
            file_name,
            symbol
        }
    }
}

impl ConvertModel {
    pub fn get_file_name(&self) -> &str {
        &self.file_name[..]
    }
    pub fn get_symbol(&self) -> &str {
        &self.symbol[..]
    }
}
