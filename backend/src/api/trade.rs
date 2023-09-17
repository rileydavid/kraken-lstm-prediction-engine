use crate::model::trade::Trade;
//use crate::model::task::TaskState;
use crate::repository::postgresdb::PostgresRepository;
//use chrono::NaiveDateTime;

use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};

use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Interval {
    interval: String,
}

#[derive(Debug, Display)]
pub enum TradeError {
    TradeNotFound,
    BadTradeRequest,
}

impl ResponseError for TradeError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TradeError::TradeNotFound => StatusCode::NOT_FOUND,
            TradeError::BadTradeRequest => StatusCode::BAD_REQUEST,
        }
    }
}

#[get("/trades/{interval}")]
pub async fn get_trades(
    interval: Path<Interval>,
    db: Data<PostgresRepository>,
) -> Result<Json<Vec<Trade>>, TradeError> {
    match db.get_trades(&interval.interval).await {
        Some(trades) => Ok(Json(trades)),
        None => Err(TradeError::BadTradeRequest)
    }
}
