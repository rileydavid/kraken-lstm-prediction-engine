use repository::repository::{import_repository, symbol_repository};
use std::path::PathBuf;
use std::{env, fs};
use tracing::{error, info};
use utils::core::postgresdb::TxAsync;
use utils::error::service_error::ServiceError;
use utils::{core::postgresdb::Tx, error::generic_error::GenericError};

// for converting
use chrono::NaiveDateTime;
use std::fs::read_to_string;
use std::fs::File;
use std::fs::Permissions;
use std::io::prelude::*;
use std::os::unix::fs::PermissionsExt;

const EXPORT: &str = "../export/";

pub async fn export_file(symbol: String) -> Result<String, GenericError> {
   
}


fn convert(file_path: PathBuf, symbol_id: i32) -> Result<(), GenericError> {
    let mut file = match File::create(EXPORT.to_owned() + "export.csv") {
        Ok(it) => it,
        Err(err) => {
            return Err(ServiceError::general_error(err.to_string()));
        }
    };

    let _ = file.set_permissions(Permissions::from_mode(0o777));

    for line in read_to_string(file_path).unwrap().lines() {
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
