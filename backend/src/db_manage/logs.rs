use crate::db_manage::errors::{DbError, NoLogSnafu, SqlxSnafu};
use rocket_db_pools::sqlx::FromRow;
use rocket_db_pools::Connection;
use serde::{Deserialize, Serialize};
use snafu::ResultExt;
use sqlx::SqliteConnection;

use crate::Db;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Log {
    pub id: i64,
    pub note_id: i64,
    pub timestamp: String,
    pub kind: String,
    pub message: String,
    pub data: Option<Vec<u8>>,
}

pub async fn create_log(
    db: &mut SqliteConnection,
    note_id: i64,
    kind: String,
    message: String,
    data: Option<Vec<u8>>,
) -> Result<i64, DbError> {
    sqlx::query(
        r#"
      INSERT INTO logs (note_id, kind, message, blob_data)
      VALUES (?, ?, ?, ?)
      "#,
    )
    .bind(note_id)
    .bind(kind)
    .bind(message)
    .bind(data)
    .execute(&mut *db)
    .await
    .context(SqlxSnafu {
        task: "creating log",
    })?;

    // Now get the last inserted row ID
    let row: (i64,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&mut *db)
        .await
        .context(SqlxSnafu {
            task: "getting created log",
        })?;

    Ok(row.0)
}

pub async fn get_log(
    db: &mut Connection<Db>,
    log_id: i64,
) -> Result<Option<Log>, DbError> {
    let log = sqlx::query_as::<_, Log>(
        r#"
        SELECT note_id, created_at, kind, message, data
        FROM logs
        WHERE id = ?
        "#,
    )
    .bind(log_id)
    .fetch_optional(&mut ***db)
    .await
    .context(NoLogSnafu { id: log_id })?;

    Ok(log)
}

pub async fn get_logs_from_note(
    db: &mut Connection<Db>,
    note_id: i64,
) -> Result<Vec<Log>, DbError> {
    let result = sqlx::query_as::<_, Log>(
        r#"SELECT id, note_id, created_at as timestamp, kind, message, blob_data as data
        FROM logs WHERE note_id = ? ORDER BY created_at"#,
    )
    .bind(note_id)
    .fetch_all(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "getting logs",
    })?;
    Ok(result)
}
