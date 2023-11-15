use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ImportModel {
    file_name: String,
    symbol: String
}

impl ImportModel {
    pub fn new(
        file_name: String,
        symbol: String,
    ) -> ImportModel {
        ImportModel {
            file_name,
            symbol
        }
    }
}

impl ImportModel {
    pub fn get_file_name(&self) -> &str {
        &self.file_name[..]
    }
    pub fn get_symbol(&self) -> &str {
        &self.symbol[..]
    }
}
