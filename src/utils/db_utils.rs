use std::collections::HashMap;
use sqlx::MySqlPool;
use crate::models::auth::User;
use crate::models::characters::Character;

pub async fn load_friends_map(user_id: Option<i32>, pool: &MySqlPool) -> Result<HashMap<i32, Vec<User>>, sqlx::Error    >  {
    // fetch all friendships
    let rows = sqlx::query!(
        r#"
        SELECT f.user_id, u.id AS friend_id, u.username AS friend_username, u.password AS friend_password,
               u.created_at AS friend_created_at, u.times_logged_in AS friend_times_logged_in,
               c.id AS friend_character_id, c.name AS friend_character_name, c.health AS friend_character_health, c.strength AS friend_character_strength
        FROM friends f
        INNER JOIN users u ON f.friend_id = u.id
        LEFT JOIN characters c ON u.character_id = c.id
        "#
    ).fetch_all(pool).await?;

    let mut result: HashMap<i32, Vec<User>> = HashMap::new();

    for row in rows {
        let friend = User {
            id: row.friend_id,
            username: row.friend_username,
            password: row.friend_password,
            created_at: row.friend_created_at,
            times_logged_in: row.friend_times_logged_in,
            character: if let Some(character_id) = row.friend_character_id {
                Some(
                    Character {
                        id: character_id,
                        name: row.friend_character_name,
                        health: row.friend_character_health,
                        strength: row.friend_character_strength
                    }
                )
            } else {
                None
            },
            character_id: row.friend_character_id,
            friends: Vec::new(),
        };

        result.entry(row.user_id).or_insert_with(Vec::new).push(friend);
    }

    Ok(result)
}