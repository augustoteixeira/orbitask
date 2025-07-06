use rocket_db_pools::sqlx::SqlitePool;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Copies the SQL dump into a new SQLite database file.
pub async fn prepare_test_db() -> SqlitePool {
    let db_path = "test.sqlite";

    if Path::new(db_path).exists() {
        fs::remove_file(db_path).unwrap();
    }

    // Load init.sql
    let output = Command::new("sqlite3")
        .arg(db_path)
        .arg(".read tests/meta.sql")
        .output()
        .expect("Failed to run sqlite3");

    assert!(
        output.status.success(),
        "Failed to prepare meta table: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Load test_dump.sql
    let output = Command::new("sqlite3")
        .arg(db_path)
        .arg(".read migrations/001-init.sql")
        .output()
        .expect("Failed to run sqlite3");

    assert!(
        output.status.success(),
        "Failed to prepare initial migration: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Load test_dump.sql
    let output = Command::new("sqlite3")
        .arg(db_path)
        .arg(".read tests/test_dump.sql")
        .output()
        .expect("Failed to run sqlite3");
    assert!(
        output.status.success(),
        "Failed to prepare test database: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    SqlitePool::connect(&format!("sqlite://{}", db_path))
        .await
        .expect("Failed to connect to test database")
}
