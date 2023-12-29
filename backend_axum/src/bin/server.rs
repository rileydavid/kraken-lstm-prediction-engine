use collector;
use dotenv::dotenv;
use tracing::info;
use utils::core::logger;
use webserver;

#[tokio::main(flavor = "multi_thread", worker_threads = 6)]
async fn main() -> Result<(), ()> {
    dotenv().ok();
    logger::run().await;
    info!("server is starting up ...");

    // test cache connection
    //cache::get_connection().await.expect("Failed to create cache connection");
    
    collector::collector::run().await;
    webserver::webserver::run().await;
    Ok(())
}
