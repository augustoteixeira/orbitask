use std::fs;

use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Database;

pub mod login;
pub use login::{get_password, set_password};
pub mod notes;
pub use notes::{create_note, get_child_notes, get_note, Note};
pub mod attributes;
pub use attributes::create_attribute;
pub mod codes;
pub use codes::create_code;
pub mod logs;
pub use logs::create_log;
pub mod errors;

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::SqlitePool);

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
