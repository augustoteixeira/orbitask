mod api;
mod db_manage;
mod frontend;
mod utils;

#[macro_use]
extern crate rocket;

use db_manage::Db;
use utils::RateLimiter;

use bcrypt::{hash, DEFAULT_COST};
use rocket::figment::Figment;
use rocket::fs::FileServer;
use rocket::response::{Flash, Redirect};
use rocket::Config;
use rocket_db_pools::{sqlx, Database};
use rpassword::prompt_password;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Database error"))]
    DbError { source: sqlx::Error },
    #[snafu(display("Rocked error"))]
    RocketError { source: rocket::Error },
}

fn rocket_config() -> Figment {
    // initialize secret key if not yet done
    let secret_key = utils::load_or_generate_secret();
    Config::figment().merge(("secret_key", secret_key))
}

#[catch(401)]
fn unauthorized() -> Flash<Redirect> {
    Flash::error(Redirect::to(uri!("/login")), "You must login first!")
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    // setup rocket and db
    let rocket = rocket::custom(rocket_config())
        .manage(RateLimiter::new())
        .attach(Db::init())
        .mount("/static", FileServer::from("static"))
        .mount("/", routes![api::login_submit])
        .mount("/", routes![api::logout_submit])
        .mount("/", routes![api::create_board_submit])
        .mount("/", routes![api::move_state_api])
        .mount("/", routes![api::create_state_submit])
        .mount("/", routes![api::create_note_submit])
        .mount("/", routes![api::delete_state_api])
        .mount("/", routes![api::rename_state_api])
        .mount("/", routes![frontend::login::login])
        .mount("/", routes![frontend::board::boards])
        .mount("/", routes![frontend::board::board])
        .mount("/", routes![frontend::board::board_settings])
        .mount("/", routes![frontend::board::new_board])
        .mount("/", routes![frontend::board::new_note])
        .register("/", catchers![unauthorized])
        .ignite()
        .await
        .context(RocketSnafu)?;
    let db = Db::fetch(&rocket).expect("Database not initialized");
    db_manage::migrate(&db)
        .await
        .expect("Failed to run DB setup");

    let mut conn = db.acquire().await.context(DbSnafu)?;
    let is_initialized =
        db_manage::get_password(&mut conn).await.clone().is_some();

    // initialize password if not yet done
    if !is_initialized {
        let password = prompt_password("Enter new password: ")
            .expect("Failed to read password from terminal");

        if !utils::is_password_valid(&password) {
            panic!("Password should be alpha-numeric only.");
        }

        let confirm = prompt_password("Confirm password: ")
            .expect("Failed to read password confirmation");

        if password != confirm {
            panic!("Passwords do not match.");
        }

        let hash =
            hash(password, DEFAULT_COST).expect("Failed to hash password");

        db_manage::set_password(&db, hash).await.context(DbSnafu)?;
    }

    rocket.launch().await.context(RocketSnafu)?;

    Ok(())
}
