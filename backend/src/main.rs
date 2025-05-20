#[macro_use]
extern crate rocket;

use rocket_db_pools::sqlx::{self, Row};
use rocket_db_pools::{Connection, Database};

#[derive(Database)]
#[database("db")]
struct Db(sqlx::SqlitePool);

#[get("/<id>")]
async fn read(mut db: Connection<Db>, id: i64) -> Option<String> {
    sqlx::query("SELECT content FROM logs WHERE id = ?")
        .bind(id)
        .fetch_one(&mut **db)
        .await
        .and_then(|r| Ok(r.try_get(0)?))
        .ok()
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let rocket = rocket::build()
        .attach(Db::init())
        .mount("/", routes![read])
        .ignite()
        .await?;

    let db = Db::fetch(&rocket).expect("Database not initialized");
    run_db_setup(&db).await.expect("Failed to run DB setup");

    rocket.launch().await?;

    Ok(())
}

async fn run_db_setup(conn: &Db) -> Result<(), sqlx::Error> {
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

            sqlx::query(
                r#"
                CREATE TABLE tasks (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    title TEXT NOT NULL,
                    status TEXT NOT NULL,
                    due_date TEXT,
                    recurrence_rule TEXT
                );
                "#,
            )
            .execute(&**conn)
            .await?;

            // Update version
            sqlx::query(
                "INSERT OR REPLACE INTO meta (key, value) VALUES ('schema_version', '1');",
            )
            .execute(&**conn)
            .await?;
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
