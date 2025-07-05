use crate::sqlx::FromRow;
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;
use snafu::ResultExt;

use super::errors::{DbError, NoNoteSnafu};
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
    println!("{parent_id:?}, {title:?}, {description:?}, {code_name:?}");
    sqlx::query(
        r#"
      INSERT INTO notes (parent_id, title, description, code_name)
      VALUES (?, ?, ?, ?)
      "#,
    )
    .bind(&parent_id)
    .bind(&title)
    .bind(&description)
    .bind(&code_name)
    .execute(&mut ***db)
    .await?;

    // Now get the last inserted row ID
    let row: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&mut ***db)
        .await?;

    let inserted_id = row.0;

    //let new_note_id: i64 = row.get("id");
    Ok(inserted_id)
}

pub async fn get_note(
    db: &mut Connection<Db>,
    note_id: i64,
) -> Result<Option<Note>, DbError> {
    let note = sqlx::query_as::<_, Note>(
        r#"
        SELECT id, parent_id, title, description, code_name
        FROM notes
        WHERE id = ?
        "#,
    )
    .bind(note_id)
    .fetch_optional(&mut ***db)
    .await
    .context(NoNoteSnafu { id: note_id })?;

    Ok(note)
}

pub async fn get_child_notes(
    db: &mut Connection<Db>,
    note_id: i64,
) -> Result<Vec<Note>, sqlx::Error> {
    let notes = sqlx::query_as::<_, Note>(
        "SELECT id, parent_id, title, description, code_name FROM notes WHERE parent_id = ? ORDER BY id"
    )
    .bind(note_id)
    .fetch_all(&mut ***db)
    .await?;

    Ok(notes)
}

pub async fn get_root_notes(
    db: &mut Connection<Db>,
) -> Result<Vec<Note>, sqlx::Error> {
    let notes = sqlx::query_as::<_, Note>(
        "SELECT id, parent_id, title, description, code_name FROM notes WHERE parent_id IS NULL ORDER BY id"
    )
    .fetch_all(&mut ***db)
    .await?;

    Ok(notes)
}
