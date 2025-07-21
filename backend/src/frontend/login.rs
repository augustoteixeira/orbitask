use super::view::{MyFlash, View, ViewState};
use rocket::get;
use rocket::request::FlashMessage;

#[get("/login")]
pub async fn login(flash: Option<FlashMessage<'_>>) -> View {
    return View {
        state: ViewState::Login,
        flash: flash.into_iter().map(MyFlash::from).collect(),
    };
}
