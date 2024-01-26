use crate::modelexecution::prediction_dto::PredictionResponseDto;
use crate::ohlc::ohlc_dto::OhlcDto;
use chrono::DateTime;
use chrono::Utc;

use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use repository::domain::model_config::ModelConfigModel;
use repository::domain::ohlc::OhlcModel;
use repository::repository::model_config_repository;
use repository::repository::ohlc_repository;
use tracing::info;
use utils::core::modelexecution_config::get_modelservice_endpoint;
use utils::error::generic_error::GenericError;
use utils::error::service_error::ServiceError;

use super::prediction_dto::PredictionRequestDto;

fn get_timestep_count(model_config: &ModelConfigModel) -> i64 {
    let look = model_config.get_lookback();
    let max_lag_span = model_config
        .get_lags()
        .iter()
        .chain(model_config.get_span_sizes().iter())
        .chain(model_config.get_window_sizes().iter())
        .max()
        .copied()
        .unwrap_or_default();
    
    (max_lag_span + look) as i64
}

fn is_valid_dataset(dataset: &Vec<OhlcModel>, timesteps: i64) -> bool {
    match dataset.len() == (timesteps as usize) + 1 {
        true => true,
        false => false,
    }
}

fn get_from_date(timesteps: &i64, from_date: &DateTime<Utc>) -> DateTime<Utc> {  
    *from_date - chrono::Duration::hours(*timesteps)
}

pub async fn execute_model(
    pool: &sqlx::PgPool,
    from_date: DateTime<Utc>,
    symbol_id: i32,
) -> Result<PredictionResponseDto, GenericError> {
    info!(
        "Incoming Request: execute_model - fromDate {:?}, symbol_id {:?}",
        from_date, symbol_id
    );

    // fetch model config + data
    let model_config = model_config_repository::get_model_config(pool, symbol_id).await.unwrap();
    let timesteps = get_timestep_count(&model_config);
    let start_date = get_from_date(&timesteps, &from_date);
    let result_data = ohlc_repository::get_ohlc_hour_range_reduced(pool, symbol_id, start_date, from_date).await.unwrap();

    // check if dataset is valid
    if is_valid_dataset(&result_data, timesteps) == false {
        return Err(ServiceError::general_error("Invalid dataset: Not enough timesteps avaliable".to_string()));
    }

    let mut request = PredictionRequestDto::from(model_config);
    request.set_from_date(from_date);
    request.set_data(
        result_data.into_par_iter()
            .map(|model| OhlcDto::from(&model))
            .collect(),
    );

    let client = reqwest::Client::new();
    let res = client
        .post(get_modelservice_endpoint())
        .json(&request)
        .send()
        .await;

    match res {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<PredictionResponseDto>().await {
                    Ok(result) => Ok(result),
                    Err(_) => Err(ServiceError::general_error("Failed to parse response".to_string())),
                }
            } else {
                Err(ServiceError::general_error(response.status().to_string()))
            }
        }
        Err(err) => Err(ServiceError::general_error(err.to_string())),
    }
}   