use crate::db_manage;
use crate::utils::RateLimiter;
use crate::Db;

use bcrypt::verify;
use std::net::IpAddr;
use std::time::Duration;

use rocket::form::Form;
use rocket::http::{Cookie, CookieJar, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::response::{Flash, Redirect};
use rocket::Request;
use rocket::State;
use rocket_db_pools::Connection;

#[derive(Debug)]
pub struct User {}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let jar = req.cookies();
        if let Some(cookie) = jar.get_private("user_id") {
            if cookie.value().to_string() == "admin" {
                Outcome::Success(User {})
            } else {
                Outcome::Forward(Status::Ok)
            }
        } else {
            Outcome::Forward(Status::Ok)
        }
    }
}

pub fn require_auth(user: Option<User>) -> Result<User, Redirect> {
    user.ok_or_else(|| Redirect::to(uri!("/login")))
}

#[get("/dashboard")]
pub fn dashboard(user: Option<User>) -> Result<String, Redirect> {
    require_auth(user)?;
    Ok(format!("Welcome!"))
}

#[get("/dashboard", rank = 2)]
pub fn dashboard_redirect() -> Redirect {
    Redirect::to(uri!("/login"))
}

fn ok_or_redirect(next: Option<String>) -> Redirect {
    let target = match next {
        Some(ref n) if n.starts_with('/') => n.clone(),
        _ => "/".to_string(),
    };
    Redirect::to(target)
}

#[derive(FromForm)]
pub struct LoginForm {
    password: String,
}

#[post("/login?<next>", data = "<form>")]
pub async fn login_submit(
    form: Form<LoginForm>,
    mut db: Connection<Db>,
    jar: &CookieJar<'_>,
    ip: IpAddr,
    limiter: &State<RateLimiter>,
    next: Option<String>,
) -> Result<Redirect, Flash<Redirect>> {
    let ip_str = ip.to_string();
    if limiter.too_many_attempts(&ip_str, 5, Duration::from_secs(600)) {
        return Err(Flash::error(
            Redirect::to("/login"),
            "Too many login attempts. Please wait.",
        ));
    }
    let LoginForm { password } = form.into_inner();

    let stored_hash = match db_manage::get_password(&mut db).await {
        Some(hash) => hash,
        None => {
            return Err(Flash::error(
                Redirect::to("/login"),
                "No password set",
            ));
        }
    };

    if verify(&password, &stored_hash).expect("Could not verify password") {
        jar.add_private(Cookie::new("user_id", "admin"));
        Ok(ok_or_redirect(next))
    } else {
        Err(Flash::error(Redirect::to("/"), "Invalid credentials."))
    }
}
