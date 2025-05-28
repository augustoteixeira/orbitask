use crate::sqlx::{FromRow, Row};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;

use super::Db;

#[derive(Debug, FromRow)]
pub struct Note {
    pub id: i64,
    pub board_id: i64,
    pub state_id: i64,
    pub name: String,
    pub start_date: String, // Consider using `chrono::NaiveDate` later
    pub due_date: String,
}

pub async fn get_notes_for_state(
    db: &mut Connection<Db>,
    state_id: i64,
) -> Result<Vec<Note>, sqlx::Error> {
    let notes = sqlx::query_as::<_, Note>(
        r#"
        SELECT id, board_id, state_id, name, start_date, due_date
        FROM notes
        WHERE state_id = ?
        ORDER BY due_date
        "#,
    )
    .bind(state_id)
    .fetch_all(&mut ***db) // Triple deref to get &mut SqliteConnection
    .await?;

    Ok(notes)
}

pub async fn create_note(
    db: &mut Connection<Db>,
    board_id: i64,
    state_id: i64,
    name: String,
    start_date: String,
    due_date: String,
    template_id: Option<i64>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO notes (board_id, state_id, name, start_date, due_date)
        VALUES (?, ?, ?, ?, ?)
        RETURNING id
        "#,
    )
    .bind(board_id)
    .bind(state_id)
    .bind(name)
    .bind(start_date)
    .bind(due_date)
    .fetch_one(&mut ***db)
    .await?;

    let id: i64 = row.try_get("id")?;
    Ok(id)
}
