use crate::sqlx::FromRow;
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;

use super::Db;

#[derive(Debug, FromRow)]
pub struct Note {
    pub id: i64,
    pub board_id: i64,
    pub factory_id: Option<i64>,
    pub state_id: i64,
    pub name: String,
    pub start_date: String, // Consider using `chrono::NaiveDate` later
    pub due_date: String,
}

pub async fn get_note_for_state(
    db: &mut Connection<Db>,
    board_id: i64,
) -> Result<Vec<Note>, sqlx::Error> {
    let notes = sqlx::query_as::<_, Note>(
        r#"
        SELECT id, board_id, factory_id, state_id, name, start_date, due_date
        FROM notes
        WHERE board_id = ?
        ORDER BY due_date
        "#,
    )
    .bind(board_id)
    .fetch_all(&mut ***db) // Triple deref to get &mut SqliteConnection
    .await?;

    Ok(notes)
}
