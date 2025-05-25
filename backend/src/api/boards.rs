use crate::db_manage::boards::create_board;
use crate::Db;

use crate::api::require_auth;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;

use super::User;

#[derive(FromForm)]
pub struct NewBoardForm {
    name: String,
    is_template: Option<bool>,
    mode: String,
    template_id: Option<i64>,
}

#[post("/boards/create", data = "<form>")]
pub async fn create_board_submit(
    user: Option<User>,
    mut db: Connection<Db>,
    form: Form<NewBoardForm>,
) -> Result<Redirect, Flash<Redirect>> {
    match require_auth(user) {
        Ok(_) => {}
        Err(redirect) => {
            return Err(Flash::error(redirect, "You must be logged in."))
        }
    }

    let form = form.into_inner();
    let is_template = form.is_template.unwrap_or(false);

    // Handle mode
    let template_id = match form.mode.as_str() {
        "template" => form.template_id,
        _ => None,
    };

    match create_board(&mut db, form.name, is_template, template_id).await {
        Ok(new_id) => Ok(Redirect::to(format!("/boards/{}", new_id))),
        Err(err) => {
            eprintln!("Error creating board: {err}");
            Err(Flash::error(
                Redirect::to("/boards/new"),
                "Could not create board",
            ))
        }
    }
}
