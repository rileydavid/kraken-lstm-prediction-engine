use axum::extract::Path;
use service::convert::convert_service;
use tracing::info;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;

//TODO get this to work 

// maybe just have a post endpoint and call that endpoint after moving files into folder (docker volume)

pub async fn convert(   
    Path(input): Path<(String, String)>
) -> Result<Json<String>, ResponseError> {
    info!("input.0 {:?}", input.0);
    info!("input.1 {:?}", input.1);

    let result =convert_service::convert_file(input.0, input.1).await;
    prepare_response(result)
}

