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

    let (sender, receiver) = mpsc::channel(1);

    collector::collector::run(receiver).await;
    webserver::webserver::run(sender).await;
    Ok(())
}
