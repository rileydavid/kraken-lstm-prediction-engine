use axum::extract::Multipart;
use axum::extract::Path;
use axum::extract::State;
use axum::response::Html;
use service::import::import_service;
use sqlx::PgPool;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::BufWriter;
use tracing::info;
use crate::utils::error::prepare_response;
use crate::utils::error::ResponseError;
use axum::Json;



pub async fn import(   
    State(pool): State<PgPool>,
    Path(input): Path<(String, String)>
) -> Result<Json<String>, ResponseError> {
    prepare_response(import_service::import_file(&pool,input.0, input.1).await)
}

pub async fn upload(
    mut multipart: Multipart
) -> Result<Json<String>, ResponseError> {
    //TODO maybe move this to service
    info!("Incoming Request: upload");
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let file_name = field.file_name().unwrap().to_string();

        println!("name: {}, file_name: {}", name, file_name);

        let file_path = format!("../import/{}", file_name);
        let file = File::create(file_path).await.unwrap();
        let mut writer = BufWriter::new(file);

        // Write each chunk to the file
        while let Some(chunk) = field.chunk().await.unwrap() {
            //info!("writing chunk of size {}", chunk.len());
            writer.write_all(&chunk).await.unwrap();
        }

        writer.flush().await.unwrap();
    }

    Ok(Json("Ok".to_string()))
}

