use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::Flash;
use rocket::Request;

pub mod login;
pub use login::{login_submit, logout_submit, require_auth};
pub mod boards;
pub use boards::create_board_submit;
pub mod states;
pub use states::move_state_api;

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

pub struct Authenticated;

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authenticated {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, ()> {
        let jar = req.cookies();
        if let Some(cookie) = jar.get_private("user_id") {
            if cookie.value().to_string() == "admin" {
                Outcome::Success(Authenticated)
            } else {
                Outcome::Forward(Status::Ok)
            }
        } else {
            Outcome::Forward(Status::Ok)
        }
    }
}
