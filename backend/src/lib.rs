pub mod api;
pub mod db_manage;
pub mod frontend;
pub mod utils;

use db_manage::Db;
use rocket::figment::Figment;
use rocket::fs::FileServer;
use rocket::uri;
use rocket::{catch, catchers, routes, Build, Rocket};
use rocket_db_pools::Database;
use utils::RateLimiter;

#[catch(401)]
pub fn unauthorized() -> rocket::response::Flash<rocket::response::Redirect> {
    rocket::response::Flash::error(
        rocket::response::Redirect::to(uri!("/login")),
        "You must login first!",
    )
}

#[catch(500)]
pub fn internal_error(
    req: &rocket::Request,
) -> rocket::response::Flash<rocket::response::Redirect> {
    println!("Request:\n{:?}", req);
    rocket::response::Flash::error(
        rocket::response::Redirect::to(uri!("/login")),
        "Internal server error",
    )
}

pub fn prepare_rocket(figment: Figment) -> Rocket<Build> {
    rocket::custom(figment)
        .manage(RateLimiter::new())
        .attach(Db::init())
        .mount("/static", FileServer::from("static"))
        .mount(
            "/",
            routes![
                api::login_submit,
                api::logout_submit,
                api::create_note_submit,
                api::notes::execute_action,
                frontend::login::login,
                frontend::notes::show_note,
                frontend::notes::new_note,
                frontend::notes::root_notes,
            ],
        )
        .register("/", catchers![unauthorized])
        .register("/", catchers![internal_error])
}
