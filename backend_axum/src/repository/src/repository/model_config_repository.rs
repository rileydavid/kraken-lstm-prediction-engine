use crate::domain::model_config::ModelConfigModel;
use sqlx::postgres::PgRow;
use tracing::info;
use utils::error::generic_error::GenericError;
use utils::error::repository_error::RepositoryError;

const QUERY_SELECT_GET_MODEL_CONFIG: &str = "SELECT * FROM get_model_config($1);"; // symbol

pub async fn get_model_config(
    pool: &sqlx::PgPool,
    symbol_id: i32,
) -> Result<ModelConfigModel, GenericError> {

    info!("get_model_config: symbol_id: {}", symbol_id);

    match sqlx::query(QUERY_SELECT_GET_MODEL_CONFIG)
        .bind(symbol_id)
        .map(|row: PgRow| ModelConfigModel::from(row))
        .fetch_one(pool)
        .await
    {
        Ok(data) => {
            info!("get_model_config: {:?}", data);    
            Ok(data)
        },
        Err(err) => Err(RepositoryError::general_error(err.to_string())),
    }
}

