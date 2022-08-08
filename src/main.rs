mod bot;
use crate::bot::start_bot;
use dotenv::dotenv;
use log::{debug, error};

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    start_bot().await;
}
