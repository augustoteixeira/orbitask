use rocket_db_pools::sqlx::FromRow;
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;
use snafu::ResultExt;

use super::errors::{DbError, NoNoteSnafu, SqlxSnafu};
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
) -> Result<i64, DbError> {
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
    .await
    .context(SqlxSnafu {
        task: "creating note",
    })?;

    // Now get the last inserted row ID
    let row: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&mut ***db)
        .await
        .context(SqlxSnafu {
            task: "getting created note",
        })?;

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

pub async fn get_all_notes(
    db: &mut Connection<Db>,
) -> Result<Vec<Note>, DbError> {
    let notes = sqlx::query_as::<_, Note>(
        "SELECT id, parent_id, title, description, code_name FROM notes ORDER BY id"
    )
    .fetch_all(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "getting all notes"
    })?;

    Ok(notes)
}

pub async fn update_note(
    db: &mut Connection<Db>,
    note_id: i64,
    title: String,
    description: String,
    code_name: Option<String>,
) -> Result<(), DbError> {
    sqlx::query(
        r#"
        UPDATE notes
        SET title = ?, description = ?, code_name = ?
        WHERE id = ?
        "#,
    )
    .bind(title)
    .bind(description)
    .bind(code_name)
    .bind(note_id)
    .execute(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "updating note",
    })?;

    Ok(())
}

pub async fn get_ancestors(
    db: &mut Connection<Db>,
    id: i64,
) -> Result<Vec<(i64, String)>, DbError> {
    let ancestors = sqlx::query_as::<_, (i64, String)>(
        r#"
        WITH RECURSIVE ancestors(id, title, parent_id, depth) AS (
            SELECT id, title, parent_id, 0
            FROM notes
            WHERE id = ?

            UNION ALL

            SELECT n.id, n.title, n.parent_id, a.depth + 1
            FROM notes n
            JOIN ancestors a ON n.id = a.parent_id
        )
        SELECT id, title
        FROM ancestors
        WHERE id != ?
        ORDER BY depth DESC;
        "#,
    )
    .bind(id)
    .bind(id)
    .fetch_all(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "getting ancestors for breadcrumb",
    })?;

    Ok(ancestors)
}
