use repository::domain::convert::ConvertModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConvertDto {
    file_name: String,
    symbol: String,
}

impl ConvertDto {
    pub fn new(
        file_name: String,
        symbol: String,
    ) -> Self {
        ConvertDto{
            file_name,
            symbol,
        }
    }
}


impl From<ConvertModel> for ConvertDto {
    fn from(value: ConvertModel) -> Self {
        ConvertDto::new(
            value.get_file_name().to_owned(),
            value.get_symbol().to_owned(),
        )
    }
}

impl From<ConvertDto> for ConvertModel {
    fn from(value: ConvertDto) -> Self {
        ConvertModel::new(
            value.file_name,
            value.symbol,
        )
    }
}
