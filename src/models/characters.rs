use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: String,
    pub health: i32,
    pub strength: i32,
}