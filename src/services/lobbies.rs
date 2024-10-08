use sqlx::MySqlPool;
use crate::models::lobbies::{Lobby, LobbyState};

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

pub async fn get_lobby_by_id(id: i32, pool: &MySqlPool) -> Result<Option<Lobby>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, name, state, max_players
        FROM lobbies
        WHERE id = ?
        "#,
        id
    ).fetch_optional(pool).await?;

    let lobby = row.map(|row| {
        Lobby::new(Some(row.id), Some(row.name), Some(row.state), Some(row.max_players))
    });

    Ok(lobby.unwrap())
}

pub async fn obtain_all_lobbies(pool: &MySqlPool) -> Result<Vec<Lobby>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, name, state, max_players
        FROM lobbies
        "#
    ).fetch_all(pool).await?;

    let lobbies = rows.into_iter().filter_map(|row| {
        Lobby::new(Some(row.id), Some(row.name), Some(row.state), Some(row.max_players))
    }).collect();

    Ok(lobbies)
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

pub async fn set_next_lobby_state(lobby_id: &i32, lobby_state: &LobbyState, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let state = LobbyState::to_i32(lobby_state) + 1;
    sqlx::query!(
        r#"
        UPDATE lobbies
        SET state=?
        WHERE id=?
        "#,
        state, lobby_id
    ).execute(pool).await?;

    Ok(())
}

pub async fn exit_user_from_lobby(user_id: &i32, pool: &MySqlPool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET lobby_id = NULL
        WHERE id = ?
        "#,
        user_id
    ).execute(pool).await?;

    Ok(())
}
