use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::MySqlPool;
use crate::models::auth::User;

pub async fn obtain_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    // obtain users list from db
    let users = sqlx::query_as::<_, User>(
        r#"
            SELECT id, username, password, created_at, times_logged_in, character_id
            FROM users
        "#
    ).fetch_all(pool).await?;

    Ok(users)
}

pub async fn obtain_user(username: &str, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
    // obtain user from db
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password, created_at, times_logged_in, character_id
        FROM users
        WHERE username = ?
        "#,
        username
    ).fetch_optional(pool).await?;

    Ok(user)
}

pub async fn update_number_of_logins(username: &str, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET times_logged_in = times_logged_in + 1
        WHERE username = ?"#,
        username
    ).execute(pool).await?;
    Ok(())
}

pub async fn get_user_by_id(user_id: i32, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
    // obtain user from db
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT id, username, password, created_at, times_logged_in, character_id
        FROM users
        WHERE id = ?
        "#,
        user_id
    ).fetch_optional(pool).await?;

    Ok(user)
}

pub async fn login_user(username: &str, password: &str, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
    //obtain user from db
    if let Some(user) = obtain_user(username, &pool).await? {
        match verify(password, &user.password) {
            Ok(true) => Ok(Some(user)),
            Ok(false) => Ok(None),
            Err(e) => {
                eprintln!("Error verifying password: {}", e);
                Err(sqlx::Error::RowNotFound)
            }
        }
    } else {
        Ok(None)
    }
}

pub async fn register_user(username: &str, password: &str, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
    // hash passed password
    let hashed_password = hash(password, DEFAULT_COST).expect("Error while hashing password");

    // check whether this username is not in db
    let users = obtain_all_users(pool).await?;

    if users.iter().find(|u| u.username == username).is_some() {
        return Ok(None);
    }

    // insert the user into the db
    sqlx::query!(
        r#"
        INSERT INTO users (username, password, character_id)
        VALUES (?, ?, 1)
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
        SELECT id, username, password, created_at, times_logged_in, character_id
        FROM users
        WHERE username = ?
        "#,
        username
    )
        .fetch_one(pool)
        .await?;

    Ok(Some(user))
}