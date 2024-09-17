use bcrypt::{hash, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use crate::models::auth::User;

pub async fn register_user(username: &str, password: &str, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
    // hash passed password
    let hashed_password = hash(password, DEFAULT_COST).expect("Error while hashing password");

    // check whether this username is not in db
    let users = obtain_all_users(pool).await?;

    if users.iter().find(|u| u.username == username).is_some() {
        return Ok(None);
    }

    // insert the user into the db
    let result = sqlx::query!(
        r#"
        INSERT INTO users (username, password)
        VALUES (?, ?)
        "#,
        username,
        hashed_password
    )
    .execute(pool)
    .await?;


    // obtain the created user from db
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password, created_at
        FROM users
        WHERE username = ?
        "#,
        username
    )
    .fetch_one(pool)
    .await?;

    Ok(Some(user))
}

pub async fn obtain_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    // obtain users list from db
    let users = sqlx::query_as::<_, User>(
        r#"
            SELECT id, username, password, created_at
            FROM users
        "#
    ).fetch_all(pool).await?;

    Ok(users)
}