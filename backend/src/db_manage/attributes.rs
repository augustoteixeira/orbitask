use crate::sqlx::{FromRow, Row};
use rocket_db_pools::Connection;

use crate::Db;

#[derive(Debug, FromRow)]
pub struct Attribute {
    pub note_id: i64,
    pub key: String,
    pub value: String,
}

pub async fn create_attribute(
    db: &mut Connection<Db>,
    note_id: i64,
    key: String,
    value: String,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
    INSERT INTO attributes (note_id, key, value)
    VALUES (?, ?, ?)
    "#,
    )
    .bind(note_id)
    .bind(key)
    .bind(value)
    .fetch_one(&mut ***db)
    .await?;

    let new_attribute_id: i64 = row.get("id");
    Ok(new_attribute_id)
}
