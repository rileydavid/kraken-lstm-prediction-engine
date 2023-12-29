use repository::domain::import::ImportModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ImportDto {
    file_name: String,
    symbol: String,
}

impl ImportDto {
    pub fn new(
        file_name: String,
        symbol: String,
    ) -> Self {
        ImportDto{
            file_name,
            symbol,
        }
    }
}

impl From<&ImportModel> for ImportDto {
    fn from(value: &ImportModel) -> Self {
        ImportDto::new(
            value.get_file_name().to_owned(),
            value.get_symbol().to_owned(),
        )
    }
}

impl From<&ImportDto> for ImportModel {
    fn from(value: &ImportDto) -> Self {
        ImportModel::new(
            value.file_name.to_owned(),
            value.symbol.to_owned(),
        )
    }
}
