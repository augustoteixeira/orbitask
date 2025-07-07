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
