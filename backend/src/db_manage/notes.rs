use crate::sqlx::{FromRow, Row};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;

use super::Db;

#[derive(Debug, FromRow)]
pub struct Note {
    pub id: i64,
    pub parent_id: Option<i64>,
    pub title: String,
    pub description: String,
    pub code_name: Option<String>,
}

pub async fn create_note(
    db: &mut Connection<Db>,
    parent_id: Option<i64>,
    title: String,
    description: String,
    code_name: Option<String>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
      INSERT INTO notes (parent_id, title, description, code_name)
      VALUES (?, ?, ?, ?)
      "#,
    )
    .bind(&parent_id)
    .bind(&title)
    .bind(&description)
    .bind(&code_name)
    .fetch_one(&mut ***db)
    .await?;

    let new_note_id: i64 = row.get("id");
    Ok(new_note_id)
}

pub async fn get_note(
    db: &mut Connection<Db>,
    note_id: i64,
) -> Result<Option<Note>, sqlx::Error> {
    let note = sqlx::query_as::<_, Note>(
        r#"
        SELECT id, parent_id, title, description, code_name
        FROM notes
        WHERE id = ?
        "#,
    )
    .bind(note_id)
    .fetch_optional(&mut ***db)
    .await?;

    Ok(note)
}
