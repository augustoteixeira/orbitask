use crate::sqlx::FromRow;
use rocket_db_pools::Connection;
use snafu::ResultExt;

use crate::Db;

use super::errors::{DbError, SqlxSnafu};

#[derive(Debug, FromRow)]
pub struct Attribute {
    pub note_id: i64,
    pub key: String,
    pub value: String,
}

pub async fn set_attribute(
    db: &mut Connection<Db>,
    note_id: i64,
    key: &str,
    value: &str,
) -> Result<(), DbError> {
    sqlx::query(
        r#"
        INSERT INTO attributes (note_id, key, value)
        VALUES (?, ?, ?)
        ON CONFLICT(note_id, key) DO UPDATE SET value = excluded.value
        "#,
    )
    .bind(note_id)
    .bind(key)
    .bind(value)
    .execute(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "setting attributes",
    })?;

    Ok(())
}

pub async fn get_attribute(
    db: &mut Connection<Db>,
    note_id: i64,
    key: &str,
) -> Result<Option<String>, DbError> {
    let result = sqlx::query_scalar::<_, String>(
        "SELECT value FROM attributes WHERE note_id = ? AND key = ?",
    )
    .bind(note_id)
    .bind(key)
    .fetch_optional(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "getting attributes",
    })?;

    Ok(result)
}

pub async fn get_attributes(
    db: &mut Connection<Db>,
    note_id: i64,
) -> Result<Vec<(String, String)>, DbError> {
    let result = sqlx::query_as::<_, (String, String)>(
        "SELECT key, value FROM attributes WHERE note_id = ?",
    )
    .bind(note_id)
    .fetch_all(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "getting attributes",
    })?;

    Ok(result)
}
