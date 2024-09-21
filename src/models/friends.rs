use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Friends {
    pub user_id: i32,
    pub friend_id: i32,
}