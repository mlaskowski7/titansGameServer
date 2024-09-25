use sqlx::MySqlPool;
use crate::models::lobbies::Lobby;

pub async fn add_new_lobby(name: &str, state: i32, max_players: i32, pool: &MySqlPool) -> Result<Lobby, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO lobbies (name, state, max_players)
        VALUES (?, ?, ?)
        "#,
        name,
        state,
        max_players
    )
    .execute(pool).await?;

    let lobby = obtain_lobby(name.to_string(), pool).await?;
    match lobby {
        Some(lobby) => Ok(lobby),
        None => Err(sqlx::Error::RowNotFound),
    }
}

pub async fn obtain_lobby(name: String, pool: &MySqlPool) -> Result<Option<Lobby>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, name, state, max_players
        FROM lobbies
        WHERE name = ?
        "#,
        name
    ).fetch_optional(pool).await?;

    let lobby = row.map(|row| {
        Lobby::new(Some(row.id), Some(row.name), Some(row.state), Some(row.max_players))
    });

    Ok(lobby.unwrap())
}

pub async fn update_user_lobby(user_id: i32, lobby_id: i32, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET lobby_id = ?
        WHERE id = ?
        "#,
        lobby_id, user_id
    ).execute(pool).await?;

    Ok(())
}