use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct Character {
    pub id: i32,
    pub name: Option<String>,
    pub health: Option<i32>,
    pub strength: Option<i32>,
}