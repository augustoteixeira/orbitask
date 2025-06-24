use crate::sqlx::{FromRow, Row};
use rocket_db_pools::Connection;

use crate::Db;

#[derive(Debug, FromRow)]
pub struct Log {
    pub id: i64,
    pub note_id: i64,
    pub timestamp: String,
    pub kind: String,
    pub message: String,
    pub data: Option<Vec<u8>>,
}

pub async fn create_log(
    db: &mut Connection<Db>,
    note_id: i64,
    kind: String,
    message: String,
    data: Option<Vec<u8>>,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
    INSERT INTO logs (note_id, kind, message, data)
    VALUES (?, ?, ?, ?)
    "#,
    )
    .bind(note_id)
    .bind(kind)
    .bind(message)
    .bind(data)
    .fetch_one(&mut ***db)
    .await?;

    let new_log_id: i64 = row.get("id");
    Ok(new_log_id)
}
