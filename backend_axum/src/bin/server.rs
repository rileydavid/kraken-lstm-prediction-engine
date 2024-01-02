//use std::sync::mpsc;

use std::sync::mpsc::Receiver;

use collector;
use dotenv::dotenv;
use tracing::info;
use utils::core::logger;
use webserver;
use tokio::sync::mpsc;

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    logger::run().await;
    info!("server is starting up ...");

    // test cache connection
    //cache::get_connection().await;.expect("Failed to create cache connection");
    
    //let sub_channel: (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

    let (sender, receiver) = mpsc::channel(1);

    collector::collector::run(receiver).await;
    webserver::webserver::run(sender).await;
    Ok(())
}
