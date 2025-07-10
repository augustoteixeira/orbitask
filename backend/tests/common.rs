//use crate::integration;

use std::net::SocketAddrV4;

use rocket::{http::ContentType, http::Status, local::asynchronous::Client};

use rocket_db_pools::sqlx::SqlitePool;

use sqlx::{sqlite::SqlitePoolOptions, Executor};

pub const LOCALHOST: SocketAddrV4 =
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 8000);

use backend::db_manage::Db;
use backend::utils::RateLimiter;
use backend::{internal_error, unauthorized};
use rocket::{catchers, routes};
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

pub async fn spawn_test_rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment()
        .merge(("databases.sqlite_logs.url", "sqlite://test.sqlite"))
        .merge(("secret_key", "01234567890123456789012345678901234567890123")); // fixed key for test

    rocket::custom(figment)
        .attach(Db::init())
        .manage(RateLimiter::new())
        .mount(
            "/",
            routes![
                backend::api::login_submit,
                backend::api::logout_submit,
                backend::api::create_note_submit,
                backend::api::notes::execute_action,
                backend::frontend::login::login,
                backend::frontend::notes::show_note,
                backend::frontend::notes::new_note,
                backend::frontend::notes::root_notes,
            ],
        )
        .register("/", catchers![unauthorized])
        .register("/", catchers![internal_error])
}

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

pub async fn login_as_test_user() -> Client {
    let rocket = spawn_test_rocket().await;
    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket instance");
    {
        let response = client
            .post("/login")
            .header(ContentType::Form)
            .body("password=123")
            .remote(LOCALHOST.into())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::SeeOther, "Login failed");
    }
    client
}
