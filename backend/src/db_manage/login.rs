use crate::sqlx::pool::PoolConnection;
use crate::sqlx::Sqlite;
use rocket_db_pools::sqlx::{self, Row};

use super::Db;

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
