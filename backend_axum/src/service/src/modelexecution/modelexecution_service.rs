use crate::modelexecution::prediction_dto::PredictionResponseDto;
use chrono::DateTime;
use chrono::Utc;

use repository::repository::model_config_repository;
use tracing::info;
use utils::core::postgresdb::Tx;
use utils::core::postgresdb::TxAsync;
use utils::error::generic_error::GenericError;
use utils::error::service_error::ServiceError;

use super::prediction_dto::PredictionRequestDto;

pub async fn execute_model(start_date: DateTime<Utc>, symbol_id: i32) -> Result<PredictionResponseDto, GenericError> {   
    let mut tx = Tx::begin().await;
    match model_config_repository::get_model_config(&mut tx, symbol_id).await {
        Ok(data) => {
            Tx::commit(tx).await;
            
            let mut request = PredictionRequestDto::from(data);
            request.set_start_date(start_date);

            // TODO!: use env config here 
            let client = reqwest::Client::new();
            let res = client.post("http://127.0.0.1:7000/execute")
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