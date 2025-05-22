use std::fs;

use crate::sqlx::pool::PoolConnection;
use crate::sqlx::{FromRow, Sqlite};
use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::SqlitePool);

pub async fn get_password(conn: &mut PoolConnection<Sqlite>) -> Option<String> {
    sqlx::query("SELECT value FROM meta WHERE key = 'password_hash';")
        .fetch_one(&mut **conn)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

pub async fn set_password(conn: &Db, hash: String) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT OR REPLACE INTO meta (key, value) VALUES ('password_hash', ?)",
    )
    .bind(hash)
    .execute(&**conn)
    .await?;

    Ok(())
}

#[derive(Debug, FromRow)]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub is_template: bool,
}

pub async fn get_all_boards(
    mut db: Connection<Db>,
) -> Result<Vec<Board>, sqlx::Error> {
    let rows: Vec<Board> = sqlx::query_as::<_, Board>(
        r#"
      SELECT id, name, is_template
      FROM boards
      ORDER BY name
        "#,
    )
    .fetch_all(&mut **db)
    .await?;

    let boards = rows
        .into_iter()
        .map(|row| Board {
            id: row.id,
            name: row.name,
            is_template: row.is_template,
        })
        .collect();

    Ok(boards)
}

pub async fn get_board(mut db: Connection<Db>, id: i64) -> Option<String> {
    sqlx::query("SELECT content FROM boards WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

pub async fn migrate(conn: &Db) -> Result<(), sqlx::Error> {
    // Create meta table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS meta (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );
        "#,
    )
    .execute(&**conn)
    .await?;

    // Read current schema version
    let version: Option<String> = sqlx::query_scalar(
        "SELECT value FROM meta WHERE key = 'schema_version';",
    )
    .fetch_optional(&**conn)
    .await?;

    let version = version.unwrap_or_else(|| "0".to_string());
    println!("Current schema version: {}", version);

    match version.as_str() {
        "0" => {
            println!("Running initial migration...");

            let sql = fs::read_to_string("./migrations/001-init.sql")
                .expect("Could not read migration file");
            sqlx::query(&sql).execute(&**conn).await?;
        }

        "1" => {
            // Schema is up to date — do nothing
        }

        _ => {
            panic!("Unknown schema version: {}", version);
        }
    }

    Ok(())
}
