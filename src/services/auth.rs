use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::MySqlPool;
use crate::models::auth::User;
use crate::models::characters::Character;
use crate::utils::db_utils::load_friends_map;
//TODO: implement friends functionality. Create many to many self relationship on db and implement eager loading on the backend

pub async fn obtain_all_users(pool: &MySqlPool) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT u.id AS user_id, u.username, u.password, u.created_at, u.times_logged_in,
               c.id AS character_id, c.name AS character_name, c.health AS character_health, c.strength AS character_strength
        FROM users u
        LEFT JOIN characters c ON u.character_id = c.id
        "#
    )
    .fetch_all(pool)
    .await?;

    let friends_map = load_friends_map(None, &pool).await?;

    let users = rows
        .into_iter()
        .map(|row| {
            let character = if let Some(character_id) = row.character_id {
                Some(Character {
                    id: character_id,
                    name: row.character_name,
                    health: row.character_health,
                    strength: row.character_strength,
                })
            } else {
                None
            };

            User {
                id: row.user_id,
                username: row.username,
                password: row.password,
                created_at: row.created_at,
                times_logged_in: row.times_logged_in,
                character,  // Load Character into the User struct
                character_id: row.character_id,
                friends: friends_map.get(&row.user_id).cloned().unwrap_or_default(),
            }
        })
        .collect();

    Ok(users)
}

pub async fn obtain_user(username: &str, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT u.id AS user_id, u.username, u.password, u.created_at, u.times_logged_in,
               c.id AS character_id, c.name AS character_name, c.health AS character_health, c.strength AS character_strength
        FROM users u
        LEFT JOIN characters c ON u.character_id = c.id
        WHERE u.username = ?
        "#,
        username
    )
        .fetch_optional(pool)
        .await?;

    if let Some(row) = row {
        let character = if let Some(character_id) = row.character_id {
            Some(Character {
                id: character_id,
                name: row.character_name,
                health: row.character_health,
                strength: row.character_strength,
            })
        } else {
            None
        };

        let user = User {
            id: row.user_id,
            username: row.username,
            password: row.password,
            created_at: row.created_at,
            times_logged_in: row.times_logged_in,
            character,  // Load Character into the User struct
            character_id: row.character_id,
            friends: Vec::new(),
        };

        Ok(Some(user))
    } else {
        Ok(None)
    }
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
    let row = sqlx::query!(
        r#"
        SELECT u.id, u.username, u.password, u.created_at, u.times_logged_in,
               c.id AS "character_id?", c.name AS "character_name?", c.health AS "character_health?", c.strength AS "character_strength?"
        FROM users u
        LEFT JOIN characters c ON u.character_id = c.id
        WHERE u.id = ?
        "#,
        user_id
    ).fetch_optional(pool).await?;

    if let Some(row) = row {
        let character = if let Some(character_id) = row.character_id {
            Some(Character {
                id: character_id,
                name: row.character_name,
                health: row.character_health,
                strength: row.character_strength,
            })
        } else {
            None
        };

        let user = User {
            id: row.id,
            username: row.username,
            password: row.password,
            created_at: row.created_at,
            times_logged_in: row.times_logged_in,
            character,  // Load Character into the User struct
            character_id: row.character_id,
            friends: Vec::new()
        };

        Ok(Some(user))
    } else {
        Ok(None)
    }
}

pub async fn update_user_by_id(id: i32, update_username: &str, update_character_id: i32, pool: &MySqlPool) -> Result<Option<User>, sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET username = ?, character_id = ?
        WHERE id = ?
        "#,
        update_username, update_character_id, id
    ).execute(pool).await?;

    Ok(get_user_by_id(id, pool).await?)
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
    Ok(obtain_user(username, pool).await?)
}

pub async fn add_friend(user_id: i32, friend_id: i32, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO friends (user_id, friend_id)
        VALUES (?, ?)
        "#,
        user_id, friend_id
    ).execute(pool).await?;

    Ok(())
}