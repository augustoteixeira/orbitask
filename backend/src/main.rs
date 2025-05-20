mod db_manage;
mod utils;

#[macro_use]
extern crate rocket;

use db_manage::Db;

use bcrypt::{hash, DEFAULT_COST};
use rocket_db_pools::{Connection, Database};
use rpassword::prompt_password;
use snafu::prelude::*;

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("Database error"))]
    DbError {
        source: rocket_db_pools::sqlx::Error,
    },
    #[snafu(display("Rocked error"))]
    RocketError { source: rocket::Error },
}

// Send to API file?
#[get("/api/category/<id>")]
async fn api_category(db: Connection<Db>, id: i64) -> Option<String> {
    db_manage::category(db, id).await
}

// Send to html frontend file?
#[get("/login")]
async fn login() -> Option<String> {
    Some("Hello world!".to_string())
}

#[rocket::main]
async fn main() -> Result<(), Error> {
    // setup rocket and db
    let rocket = rocket::build()
        .attach(Db::init())
        .mount("/", routes![api_category])
        .mount("/", routes![login])
        .ignite()
        .await
        .context(RocketSnafu)?;
    let db = Db::fetch(&rocket).expect("Database not initialized");
    db_manage::migrate(&db)
        .await
        .expect("Failed to run DB setup");

    let is_initialized = db_manage::get_password(&db).await.clone().is_some();

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
