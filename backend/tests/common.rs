use rocket_db_pools::sqlx::SqlitePool;

use sqlx::{sqlite::SqlitePoolOptions, Executor};

pub async fn prepare_test_db() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect(":memory:")
        .await
        .unwrap();

    let meta = include_str!(".././tests/meta.sql");
    let migration = include_str!("../migrations/001-init.sql");
    let dump = include_str!(".././tests/test_dump.sql");
    pool.execute(meta).await.unwrap();
    pool.execute(migration).await.unwrap();
    pool.execute(dump).await.unwrap();
    pool
}
