use axum::extract::Path;
use tracing::{info, error};

//TODO get this to work 

// maybe just have a post endpoint and call that endpoint after moving files into folder (docker volume)

pub async fn convert(   
    Path(input): Path<(String, String)>
) {
 
}

