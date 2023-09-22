use crate::model::{ohlc::Ohlc, trade::Trade};
use crate::repository::postgresdb::PostgresRepository;
use crate::repository::collector::Collector;
use crate::repository::provider::Provider;

use actix_web::{web, HttpRequest};
use actix_web::{
    error::ResponseError,
    get,
    post,
    http::{header::ContentType, StatusCode},
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
    Error
};

use actix_web_actors::ws;
use derive_more::Display;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Interval {
    interval: String,
}

#[derive(Deserialize, Serialize, Clone)]
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

#[get("/ws")]
pub async fn websocket(req: HttpRequest, stream: web::Payload, collector: Data<Collector>) -> Result<HttpResponse, Error> {
    ws::start(Provider::new(collector), &req, stream)
}

#[post("/prediction")]
pub async fn new_prediction(
    collector: Data<Collector>,
    db: Data<PostgresRepository>,
    trade: web::Json<Trade>
) -> Result<Json<String>, ApiError> {
    match collector.post_trade_prediction(trade.0.clone(), db, collector.clone()).await {
        Some(response) => Ok(Json(response)),
        None => Err(ApiError::BadRequest),
    }
}


#[get("/cached_trades/{symbol}/{interval}")]
pub async fn get_cached_trades(
    collector: Data<Collector>,
    symbol: Path<Symbol>,
    interval: Path<Interval>
) -> Result<Json<Vec<Trade>>, ApiError> {
    match collector.get_cache(&symbol.symbol, &interval.interval).await {
        Some(response) => Ok(Json(response)),
        None => Err(ApiError::BadRequest),
    }
}


#[get("/cached_prediction/{symbol}/{interval}")]
pub async fn get_cached_trades_prediction(
    collector: Data<Collector>,
    symbol: Path<Symbol>,
    interval: Path<Interval>
) -> Result<Json<Vec<Trade>>, ApiError> {
    match collector.get_cached_trades_prediction(&symbol.symbol, &interval.interval).await {
        Some(response) => Ok(Json(response)),
        None => Err(ApiError::BadRequest),
    }
}


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
