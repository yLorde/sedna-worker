use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct StatusModel {
    pub id: i32,
    pub uptime: i32,
    pub latency: i32,
    pub created_at: DateTime<Utc>,
}
