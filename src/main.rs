mod bot;
use crate::bot::start_bot;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

    start_bot().await;
}
