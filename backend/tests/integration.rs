use backend::db_manage::Db;
use backend::utils::RateLimiter;
use rocket::routes;
use rocket::{Build, Rocket};
use rocket_db_pools::Database;

pub async fn spawn_test_rocket() -> Rocket<Build> {
    let figment = rocket::Config::figment()
        .merge(("databases.sqlite_logs.url", "sqlite://test.sqlite"))
        .merge(("secret_key", "01234567890123456789012345678901234567890123")); // fixed key for test

    rocket::custom(figment)
        .attach(Db::init())
        .manage(RateLimiter::new())
        .mount("/", routes![backend::frontend::notes::root_notes])
        .mount("/", routes![backend::frontend::login::login])
}
