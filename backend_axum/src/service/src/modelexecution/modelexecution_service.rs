use std::vec;

use crate::modelexecution::prediction_dto::PredictionResponseDto;
use crate::ohlc::ohlc_dto::OhlcDto;
use chrono::Date;
use chrono::DateTime;
use chrono::Utc;

use repository::repository::model_config_repository;
use repository::repository::ohlc_repository;
use tracing::info;
use utils::core::postgresdb::Tx;
use utils::core::postgresdb::TxAsync;
use utils::error::generic_error::GenericError;
use utils::error::service_error::ServiceError;
use utils::core::modelexecution_config::get_modelservice_endpoint;

use super::prediction_dto::PredictionRequestDto;


pub async fn execute_model(from_date: DateTime<Utc>, symbol_id: i32) -> Result<PredictionResponseDto, GenericError> {   
    let mut tx = Tx::begin().await;
    match model_config_repository::get_model_config(&mut tx, symbol_id).await {
        Ok(data) => {
            Tx::commit(tx).await;

            let look = data.get_lookback();
            let lags = data.get_lags().iter().max().unwrap();
            let span_sizes = data.get_span_sizes().iter().max().unwrap();
            //let window_sizes = data.get_window_sizes().iter().max().unwrap();

            let vec = vec![*look, *lags, *span_sizes];

            info!("vec: {:?}", vec.iter().max());

            info!("look: {}", look);
            info!("lags: {}", lags);
            info!("span_sizes: {}", span_sizes);
            
            info!("interval {}", *vec.iter().max().unwrap() as i64 + *look as i64);

            let start_date = from_date - chrono::Duration::hours(*vec.iter().max().unwrap() as i64 + *look as i64);

            info!("from_date: {}", from_date);
            info!("to_date: {}", start_date);
            
            let mut result_data: Vec<OhlcDto> = Vec::new();

            let mut tx = Tx::begin().await;
            match ohlc_repository::get_ohlc_hour_range(&mut tx, symbol_id, start_date, from_date).await
            {
                Ok(data) => {
                    Tx::commit(tx).await;
                    result_data = data.into_iter().map(|model| OhlcDto::from(model)).collect();
                },
                Err(err) => {
                    return Err(err);
                }
            }


            let mut request = PredictionRequestDto::from(data);
            request.set_from_date(from_date);
            request.set_data(result_data);

            let client = reqwest::Client::new();
            let res = client.post(get_modelservice_endpoint())
                .json(&request)
                .send()
                .await;
            
            match res { 
                Ok(response) => {
                    if response.status().is_success() {
                        let result: PredictionResponseDto = response.json().await.unwrap();
                        Ok(result)
                    }else{
                        Err(ServiceError::general_error(response.status().to_string()))
                    }
                },
                Err(err) => {
                    Err(ServiceError::general_error(err.to_string()))
                }
            }
        }
        Err(err) => Err(err),
    }
}