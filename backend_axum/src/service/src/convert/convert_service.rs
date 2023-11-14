use repository::repository::{symbol_repository, convert_repository};
use utils::core::postgresdb::TxAsync;
use utils::{error::generic_error::GenericError, core::postgresdb::Tx};
use utils::error::service_error::ServiceError;
use std::path::PathBuf;
use std::{fs, env};
use tracing::{error, info};

// for converting 
use chrono::NaiveDateTime;
use std::io::prelude::*;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::Permissions;
use std::os::unix::fs::PermissionsExt;

//const IMPORT_PATH: &str = "./data/import/";
// full path for now for testing
//const IMPORT_PATH: &str = "/home/rileydavid/Documents/google_drive/studium/BachelorThesis/PredictionModel/trading/backend_axum/data/import/";
const IMPORT_TIMESCALE: &str = "/import/";
const IMPORT: &str = "../import/";

pub async fn convert_file(
    file_name: String,
    symbol: String
) -> Result<String, GenericError> {

    match file_name_path(&file_name)  {
        Some(path) => {
            let mut tx = Tx::begin().await;
            let symbol_id = symbol_repository::get_symbol_id(&mut tx, symbol).await;
            
            info!("symbol_id {:?}", symbol_id);

            match convert(path, symbol_id.unwrap()){
                Ok(_) => {
                    //continue here
                    let mut tx = Tx::begin().await;
                    let result = convert_repository::copy_csv(&mut tx, IMPORT_TIMESCALE.to_owned() + "converted.csv".into()).await;
                    Tx::commit(tx).await;
                    result
                },
                Err(err) => {return Err(err)},
            }

        },
        None => {
            Err(ServiceError::general_error(format!("file_name: {:?} not found", file_name)))
        }
    }
}

fn file_name_path(file_name: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(IMPORT).unwrap() {
        if entry.is_ok() {
            if file_name == entry.as_ref().unwrap().file_name().into_string().unwrap() {
                return Some(entry.unwrap().path());
            }
        }
    }
    None
}

fn convert(file_path: PathBuf, symbol_id: i32) -> Result<(), GenericError>{
    let mut file = match File::create(IMPORT.to_owned() + "converted.csv") {
        Ok(it) => it,
        Err(err) => {
            return Err(ServiceError::general_error(err.to_string()));
        }
    };

    let _ = file.set_permissions(Permissions::from_mode(0o777));

    for line in read_to_string(file_path)
        .unwrap()
        .lines()
    {
        let mut converted: Vec<String> = Vec::with_capacity(3);

        for (index, part) in line.split(",").enumerate() {
            match index {
                0 => {
                    let time = NaiveDateTime::from_timestamp_micros(
                        (part.parse::<f64>().unwrap() * 1000000.0) as i64,
                    )
                    .unwrap()
                    .to_string();
                    converted.insert(index, time);
                }
                2 => {
                    converted.insert(
                        index,
                        part.to_string() + ",,," + &symbol_id.to_string() + "\n",
                    );
                    //&file_name.replace(".csv", "")
                }
                _ => {
                    converted.insert(index, part.to_string());
                }
            }
        }

        match file.write_all(converted.join(",").as_bytes()) {
            Ok(it) => it,
            Err(err) => return Err(ServiceError::general_error(err.to_string())),
        }
    }
    Ok(())
}