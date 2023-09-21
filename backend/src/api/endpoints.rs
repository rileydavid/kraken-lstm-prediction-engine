use crate::model::{ohlc::Ohlc, trade::Trade};
use crate::repository::postgresdb::PostgresRepository;
use crate::repository::collector::Collector;


use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
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

#[derive(Deserialize, Serialize)]
pub struct Symbol {
    symbol: String,
}

#[derive(Debug, Display)]
pub enum ApiError {
    _NotFound, // _ so it does not produce a warning for now
    BadRequest,
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::_NotFound => StatusCode::NOT_FOUND,
            ApiError::BadRequest => StatusCode::BAD_REQUEST,
        }
    }
}

/*
#[get("/web")]
pub async fn get_web(
    websocket: Data<Websocket>,
) -> Result<Json<String>, ApiError> {
    match websocket.get_web_data().await {
        Some(response) => Ok(Json(response)),
        None => Err(ApiError::BadRequest),
    }
}
 */

/*
pub async fn start_kraken_websocket (
    websocket: Data<Websocket>,
) -> Result<Json<String>, ApiError> {
    match websocket.start_kraken_websocket().await {
        Some(response) => Ok(Json(response)),
        None => Err(ApiError::BadRequest),
    }
}
 */


#[get("/close_websocket")]
pub async fn close_websocket(
    collector: Data<Collector>,
) -> Result<Json<String>, ApiError> {
    match collector.close_websocket().await {
        Some(response) => Ok(Json(response)),
        None => Err(ApiError::BadRequest),
    }
}


#[get("/symbols")]
pub async fn get_symbols(
    db: Data<PostgresRepository>,
) -> Result<Json<Vec<String>>, ApiError> {
    match db.get_symbols().await {
        Some(symbols) => Ok(Json(symbols)),
        None => Err(ApiError::BadRequest),
    }
}

#[get("/trades/{interval}/{symbol}")]
pub async fn get_trades(
    interval: Path<Interval>,
    symbol: Path<Symbol>,
    db: Data<PostgresRepository>,
) -> Result<Json<Vec<Trade>>, ApiError> {

    match db.get_trades(&interval.interval, &symbol.symbol).await {
        Some(trades) => Ok(Json(trades)),
        None => Err(ApiError::BadRequest),
    }
}

#[get("/ohlc/{interval}/{symbol}")]
pub async fn get_ohlc(
    interval: Path<Interval>,
    symbol: Path<Symbol>,
    db: Data<PostgresRepository>,
) -> Result<Json<Vec<Ohlc>>, ApiError> {
    match db.get_ohlc(&interval.interval, &symbol.symbol).await {
        Some(candles) => Ok(Json(candles)),
        None => Err(ApiError::BadRequest),
    }
}
