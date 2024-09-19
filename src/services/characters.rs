use sqlx::MySqlPool;
use crate::models::characters::Character;

pub async fn obtain_all_characters(pool: &MySqlPool) -> Result<Vec<Character>, sqlx::Error> {
    let result = sqlx::query_as!(
    Character,
    r#"
    SELECT id, name, health, strength
    FROM characters
    "#
    ).fetch_all(pool).await?;

    Ok(result)
}