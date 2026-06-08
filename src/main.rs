use std::time::Duration;

mod database;
mod ping_api;

use ping_api::ping_api;
use sqlx::{Pool, Postgres};

#[derive(Clone)]
pub struct AppState {
    pub client_db: Pool<Postgres>,
}

impl AppState {
    pub fn new(client_db: Pool<Postgres>) -> AppState {
        AppState { client_db }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting Sedna Worker...");

    dotenv::dotenv().ok();
    env_logger::init();

    let pool = database::postgres_connection::local_connect().await;

    let mut tick = tokio::time::interval(Duration::from_secs(20 * 60));
    loop {
        tick.tick().await;
        ping_api(AppState::new(pool.clone())).await;
    }
}
