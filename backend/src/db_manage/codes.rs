use crate::sqlx::{FromRow, Row};
use rocket_db_pools::Connection;

use crate::Db;

#[derive(Debug, FromRow)]
pub struct Code {
    pub name: String,
    pub code: String,
}

pub async fn create_code(
    db: &mut Connection<Db>,
    name: String,
    code: String,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
    INSERT INTO codes (name, code)
    VALUES (?, ?)
    "#,
    )
    .bind(name)
    .bind(code)
    .fetch_one(&mut ***db)
    .await?;

    let new_attribute_id: i64 = row.get("id");
    Ok(new_attribute_id)
}

pub async fn get_all_code_names(
    db: &mut Connection<Db>,
) -> Result<Vec<String>, sqlx::Error> {
    let names =
        sqlx::query_scalar::<_, String>("SELECT name FROM codes ORDER BY name")
            .fetch_all(&mut ***db)
            .await?;

    Ok(names)
}
