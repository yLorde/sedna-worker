use std::time::Duration;

mod database;
mod models;
mod services;

use sqlx::{Pool, Postgres};
use tokio::time::interval;

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
async fn main() -> anyhow::Result<()> {
    println!("Starting Sedna Worker...");

    dotenv::dotenv().ok();
    env_logger::init();

    let pool = database::postgres_connection::local_connect().await;

    // Get env vars
    let delay_time = std::env::var("DELAY_TIME")
        .expect("DELAY_TIME mut be set")
        .parse::<u64>()
        .unwrap();

    println!("Delay time: {} minutes \n", delay_time);

    // Start pings
    let ping_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(delay_time * 60));
        loop {
            tick.tick().await;
            services::heartbeat::heartbeat(AppState::new(ping_pool.clone()), delay_time).await;
        }
    });

    // Wait for 30 seconds
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Start calc latency
    let c_latency_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(delay_time * 60));
        loop {
            tick.tick().await;
            services::calc_latency::calc_latency(AppState::new(c_latency_pool.clone())).await;
        }
    });

    // Wait for 30 seconds
    tokio::time::sleep(Duration::from_secs(15)).await;

    // Start make status
    let c_status_pool = pool.clone();
    tokio::spawn(async move {
        let mut tick = interval(Duration::from_secs(delay_time * 60));
        loop {
            tick.tick().await;
            services::make_status::make_status(AppState::new(c_status_pool.clone())).await;
        }
    });

    // Exit
    tokio::signal::ctrl_c().await?;
    Ok(())
}
