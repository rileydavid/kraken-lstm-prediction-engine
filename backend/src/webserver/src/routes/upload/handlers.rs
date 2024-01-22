use axum::extract::Multipart;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::io::BufWriter;
use tracing::info;
use crate::utils::error::ResponseError;
use crate::utils::error::prepare_response;
use axum::Json;

pub async fn upload(
    mut multipart: Multipart
) -> Result<Json<String>, ResponseError> {
    info!("Incoming Request: upload");
    // TODO: create service for this
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
    
        info!("uploading file: {}", file_name);
    
        let file_path = format!("../import/{}", file_name);
        let file = File::create(file_path).await.unwrap();
        let mut writer = BufWriter::new(file);
    
        // Write each chunk to the file
        while let Some(chunk) = field.chunk().await.unwrap() {
            writer.write_all(&chunk).await.unwrap();
        }
    
        writer.flush().await.unwrap();
    }

    return prepare_response(Ok("".to_owned()));
}

