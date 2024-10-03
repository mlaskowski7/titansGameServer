use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::models::characters::Character;
use crate::models::lobbies::Lobby;

#[derive(FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub created_at: Option<DateTime<Utc>>,
    pub times_logged_in: Option<i32>,
    pub current_health: Option<i32>,
    pub character: Option<Character>,
    pub lobby: Option<Lobby>,
    pub points: Option<i32>,
    pub character_id: Option<i32>,
    pub friends: Vec<User>,
    pub lobby_id: Option<i32>,
}
