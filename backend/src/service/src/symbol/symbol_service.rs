use std::collections::HashSet;

use crate::symbol::symbol_dto::SymbolDto;
use lazy_static::lazy_static;
use rayon::prelude::*;
use repository::repository::symbol_repository;
use utils::error::{generic_error::GenericError, service_error::ServiceError};

use tracing::{error, warn};

// all symbols listed on the exchange --> move to database!
lazy_static! {
    static ref SYMBOLS: HashSet<&'static str> = {
        let mut set = HashSet::new();
        set.extend(
            [
                "ZRX", "1INCH", "AAVE", "GHST", "ACA", "AGLD", "AKT", "ALCX", "ACH", "ALGO", "TLM",
                "AIR", "ADX", "FORTH", "ANKR", "APE", "API3", "APT", "ANT", "ARB", "ARPA", "ASTR",
                "AUDIO", "REP", "REPV2", "AVAX", "AXS", "BADGER", "BAL", "BNT", "BAND", "BOND",
                "BAT", "BSX", "BICO", "BNC", "BTC", "BCH", "BIT", "BTT", "BLUR", "BLZ", "BOBA",
                "FIDA", "BRICK", "ADA", "CTSI", "CELR", "CFG", "XCN", "LINK", "CHZ", "CHR", "CVC",
                "COMP", "C98", "CVX", "ATOM", "COTI", "CQT", "CSM", "CRV", "DAI", "DASH", "MANA",
                "DENT", "DOGE", "DYDX", "EWT", "ENJ", "MLN", "EOS", "ETHW", "ETH", "ETC", "ENS",
                "EUL", "FTM", "FET", "FIL", "FLR", "FLOW", "FXS", "GALA", "GAL", "GARI", "MV",
                "GTC", "GMX", "GNO", "GST", "FARM", "HFT", "HDX", "ICX", "IDEX", "RLC", "IMX",
                "INJ", "TEER", "INTR", "ICP", "JASMY", "JUNO", "KAR", "KAVA", "KEEP", "KP3R",
                "ROOK", "KILT", "KIN", "KINT", "KSM", "KNC", "LDO", "LCX", "LMWR", "LSK", "LTC",
                "LPT", "LRC", "MNGO", "MC", "MXC", "ALICE", "MKR", "MSOL", "POND", "MASK", "MINA",
                "MIR", "XMR", "GLMR", "MOON", "MOVR", "MULTI", "EGLD", "NANO", "NEAR", "NODL",
                "NMR", "NYM", "OCEAN", "OMG", "ORCA", "OXT", "OGN", "OXY", "PARA", "PAXG", "PEPE",
                "PERP", "PHA", "PLA", "DOT", "POLS", "MATIC", "POWR", "PSTAKE", "QTUM", "QNT",
                "RARI", "RAY", "REN", "RNDR", "REQ", "XRP", "XRT", "RPL", "RBC", "SBR", "SAMO",
                "SCRT", "KEY", "SRM", "SHIB", "SDN", "SC", "SOL", "SGB", "SPELL", "STX", "ATLAS",
                "POLIS", "STG", "ALPHA", "XLM", "STEP", "GMT", "STORJ", "SUI", "SUSHI", "RAD",
                "FIS", "SUPER", "RARE", "SYN", "SNX", "TBTC", "LUNA2", "LUNA", "EURT", "UST",
                "TVK", "USDT", "XTZ", "GRT", "SAND", "RUNE", "T", "TOKE", "TRX", "TRU", "TUSD",
                "UNFI", "UNI", "UMA", "USDC", "WAVES", "WOO", "WBTC", "WAXL", "YFI", "YGG", "ZEC",
            ]
            .iter()
            .cloned(),
        );
        set
    };
}

pub async fn get_available_symbols() -> Result<Vec<SymbolDto>, GenericError> {
    let mut symbols = Vec::new();
    for symbol in SYMBOLS.iter() {
        symbols.push(SymbolDto::new(0, symbol.to_string())); // id 0 is not used
    }
    Ok(symbols)
}

pub async fn add_subscription_symbol(
    pool: &sqlx::PgPool,
    sender: &tokio::sync::mpsc::Sender<String>,
    symbol: String,
) -> Result<String, GenericError> {
    if SYMBOLS.contains(symbol.as_str()) {
        let symbol_usd = symbol + "/USD";
        // check if symbol is already in the database --> if present already subscribed
        let response = symbol_repository::get_symbol_id(pool, &symbol_usd).await;

        // 0 is not a valid id
        if response.is_ok() && response.unwrap() == 0 {
            // symbol already subscribed
            match sender.send(symbol_usd.clone()).await {
                Ok(_) => {
                    // add symbol to database
                    match symbol_repository::insert_symbol(pool, symbol_usd).await {
                        Ok(_) => {}
                        Err(err) => {
                            // can be ignored as the collector inserts the symbol
                            warn!("Failed to insert symbol to database whilst subscribing. {:?}", err);
                        }
                    }

                    return Ok("Subscribed".into());
                }
                Err(err) => {
                    error!("Failed to send symbol to channel");
                    return Err(ServiceError::general_error(err.to_string()));
                }
            }
        } else {
            return Ok("Already subscribed".into());
        }
    }
    return Err(ServiceError::general_error(
        "Failed to add new subscription".to_string(),
    ));
}

pub async fn get_symbols(pool: &sqlx::PgPool) -> Result<Vec<SymbolDto>, GenericError> {
    match symbol_repository::get_symbols(pool).await {
        Ok(data) => {
            let result: Vec<SymbolDto> = data
                .into_par_iter()
                .map(|model| SymbolDto::from(&model))
                .collect();

            Ok(result)
        }
        Err(err) => Err(err),
    }
}
