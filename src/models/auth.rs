use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub times_logged_in: Option<i32>,
    pub character_id: Option<i32>,
}
