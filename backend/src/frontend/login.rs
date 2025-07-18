use maud::{html, Markup};

use super::style::{base_flash, footer, meta};
use super::view::{View, ViewState};
use rocket::get;
use rocket::request::FlashMessage;

#[get("/login")]
pub async fn login(flash: Option<FlashMessage<'_>>) -> View {
    return View {
        state: ViewState::Login,
        flash: Vec::new(),
    };
}
