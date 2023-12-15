use chrono::NaiveDateTime;
use postgres::{Client, NoTls};
use std::fs::read_to_string;
use std::fs::File;
use std::fs::Permissions;
use std::io::prelude::*;
use std::ops::Add;
use std::os::unix::fs::PermissionsExt;
use std::time;
use std::{fs, thread};
use threadpool::ThreadPool;

fn main() {
    let data_path = r#"./data/data/"#;
    let converted_path = r#"./data/data/converted/"#;
    let ten_millis = time::Duration::from_millis(10);
    let n_workers = 7; // threads used for converions
    let pool = ThreadPool::new(n_workers);

    thread::sleep(time::Duration::from_secs(20)); //wait till timescale is ready

    let mut client = match Client::connect("postgres://admin:password@172.1.0.10:5432/db", NoTls) {
        Ok(it) => it,
        Err(_err) => {
            println!("Error can't connect");
            return;
        }
    };

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
                    pool.execute(|| convert(name, converted.to_string(), data.to_string()));
                }
            }

            // wait for all files to be done processing
            pool.join();

            let paths = fs::read_dir(data_path).unwrap();
            println!("Cleanup");
            //clean up directory
            for path in paths {
                let file = path.unwrap();
                //let file_name = file.as_os_str().to_str().unwrap().to_string();

                if !file.file_name().to_str().unwrap().contains("converted")
                    && !&file.file_name().to_str().unwrap().contains("backup")
                {
                    let _ = fs::remove_file(&file.path());
                }
            }
            println!("Cleanup done");

            // check for new files
            let mut paths = fs::read_dir(converted_path).unwrap();

            println!("Importing");
            for path in paths {
                let file = path.unwrap();
                let file_name = &file.file_name().as_os_str().to_str().unwrap().to_string();
                let query = "COPY kraken_trade (time, price, volume, side, order_type, symbol) FROM '/import/data/converted/".to_owned().add(&file_name).add("' DELIMITER ',';");
                let _ = client.batch_execute(&query.to_owned());
                let _ = fs::remove_file(file.path()); // remove file
            }
            println!("Import done");
        } else {
            thread::sleep(time::Duration::from_secs(60)) // check again in 1 min
        }
    }
}

fn convert(file_name: String, converted_path: String, data_path: String) {
    println!("Converting: {:?}", &file_name);

    let mut file = match File::create(converted_path.to_owned() + &file_name) {
        Ok(it) => it,
        Err(err) => {
            println!("Error could not open file {}, error :{}", file_name, err);
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
                        part.to_string() + ",,," + &file_name.replace(".csv", "") + "\n",
                    );
                }
                _ => {
                    converted.insert(index, part.to_string());
                }
            }
        }

        match file.write_all(converted.join(",").as_bytes()) {
            Ok(it) => it,
            Err(err) => println!("Could not save file {}", err),
        };
    }
}
