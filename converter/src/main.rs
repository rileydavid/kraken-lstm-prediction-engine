use chrono::NaiveDateTime;

use dotenv::dotenv;

use env_logger;
use log::{info, error};
use sqlx::Row;
use sqlx::postgres::PgPoolOptions;

use std::fs::read_to_string;
use std::fs::File;
use std::fs::Permissions;
use std::io::prelude::*;
use std::ops::Add;
use std::os::unix::fs::PermissionsExt;
use std::time;
use std::{fs, thread};
use threadpool::ThreadPool;

#[tokio::main]
async fn main() {
    // load env configuration --> DB URL, etc
    dotenv().ok();

    // init logging
    env_logger::init();
    info!("Starting Converter");

    let data_path = r#"./data/import/"#;
    let converted_path = r#"./data/import/converted/"#;
    let ten_millis = time::Duration::from_millis(10);
    let n_workers = 7; // threads used for converions
    let pool = ThreadPool::new(n_workers);

    let pgpool = PgPoolOptions::new()
        .max_connections(1)
        .connect("postgres://admin:password@172.1.0.10:5432/db")
        .await
        .unwrap();

    thread::sleep(time::Duration::from_secs(15)); //wait till timescale is ready

    loop {
        if fs::read_dir(data_path).unwrap().count() > 2 {
            // check if new files have been added
            let paths = fs::read_dir(data_path).unwrap();

            for path in paths {
                let file = path.unwrap().file_name();
                let file_name = file.as_os_str().to_str().unwrap().to_string();

                if !file_name.contains("converted") && !file_name.contains("backup") {
                    while pool.active_count() == n_workers {
                        //probably not the best solution
                        thread::sleep(ten_millis);
                    }
                    let name = file_name.clone();
                    let data = data_path.clone();
                    let converted = converted_path.clone();

                    // get the symbol id 
                    let rows = sqlx::query("SELECT * FROM symbols")
                    .fetch_all(&pgpool)
                    .await;
        
                    let mut symbol_id: Option<i32> = None;
    
                    if rows.is_ok() {
                        for row in rows.unwrap().iter() {
                            let current_symbol: &str = row.get(1);
                
                            if file_name.starts_with(current_symbol) {
                                symbol_id = Some(row.get(0));
                            }
                        } 
                    }

                    if symbol_id.is_none() {
                        //INSERT INTO symbols (symbol) VALUES ('ETHUSD') RETURNING symbols.id;
                        let rows = sqlx::query("INSERT INTO symbols (symbol) VALUES ($1) RETURNING symbols.id;").bind(&file_name.replace(".csv", "")).fetch_all(&pgpool).await;
                        
                        if rows.is_ok() {
                            for row in rows.unwrap() {
                                symbol_id = Some(row.get(0));
                            }
                        }
                    }

                    pool.execute(move || convert(name, converted.to_string(), data.to_string(), symbol_id.unwrap()));
                }
            }

            // wait for all files to be done processing
            pool.join();

            let paths = fs::read_dir(data_path).unwrap();
            info!("Cleanup");
            //clean up directory
            for path in paths {
                let file = path.unwrap();
                if !file.file_name().to_str().unwrap().contains("converted")
                    && !&file.file_name().to_str().unwrap().contains("backup")
                {
                    let _ = fs::remove_file(&file.path());
                }
            }
            info!("Cleanup done");

            // check for new files
            let paths = fs::read_dir(converted_path).unwrap();

            info!("Importing");
            for path in paths {
                let file = path.unwrap();
                let file_name = &file.file_name().as_os_str().to_str().unwrap().to_string();
                let query = "COPY kraken_trade (time, price, volume, side, order_type, symbol_id) FROM '/import/".to_owned().add(&file_name).add("' DELIMITER ',';");
                let _ = sqlx::query(&query).execute(&pgpool).await;
                let _ = fs::remove_file(file.path()); // remove file
            }
            info!("Import done");
        } else {
            thread::sleep(time::Duration::from_secs(60)) // check again in 1 min
        }
    }
}

fn convert(file_name: String, converted_path: String, data_path: String, symbol_id: i32) {
    let mut file = match File::create(converted_path.to_owned() + &file_name) {
        Ok(it) => it,
        Err(err) => {
            error!("Error could not open file {}, error :{}", file_name, err);
            return;
        }
    };

    let _ = file.set_permissions(Permissions::from_mode(0o777));

    for line in read_to_string(data_path.to_owned() + &file_name)
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
            Err(err) => error!("Could not save file {}", err),
        };
    }
}
