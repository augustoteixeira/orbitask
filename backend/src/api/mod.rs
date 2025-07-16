use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::response::{self, Redirect, Responder, Response};
use rocket::Request;

pub mod login;
pub use login::{login_submit, logout_submit};
pub mod attributes;
pub mod codes;
pub mod logs;
pub mod notes;
pub use notes::create_note_submit;

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
                Outcome::Forward(Status::Unauthorized)
            }
        } else {
            Outcome::Forward(Status::Unauthorized)
        }
    }
}

// pub struct RedirectFairing;

// #[rocket::async_trait]
// impl Fairing for RedirectFairing {
//     fn info(&self) -> Info {
//         Info {
//             name: "Redirect if `next` field is present",
//             kind: Kind::Response,
//         }
//     }

//     async fn on_response<'r>(
//         &self,
//         request: &'r Request<'_>,
//         response: &mut Response<'r>,
//     ) {
//         if let Some(next) = request.query_value::<String>("next") {
//             println!("AAHH");
//             let redirect_url = next.unwrap().to_string();
//             *response = Redirect::to(redirect_url).respond_to(request).unwrap();
//         }
//     }
// }
