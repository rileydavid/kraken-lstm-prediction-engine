
use axum::extract::Multipart;
use tracing::{info, error};

use crate::utils::error;

//TODO get this to work 
pub async fn file_upload(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let data = field.bytes().await;
        match data {
            Ok(data) => {
                info!("data {:?}",data);
                // process data
            },
            Err(e) => {
                // handle error, e.g., file too large
                error!("Error processing file: {}", e);
            }
        }
    }
}

