use crate::db_manage::boards::create_board;
use crate::Db;

use crate::api::Authenticated;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;

#[derive(FromForm)]
pub struct NewBoardForm {
    name: String,
    is_template: Option<bool>,
    mode: String,
    template_id: Option<i64>,
}

#[post("/boards/create", data = "<form>")]
pub async fn create_board_submit(
    _auth: Authenticated,
    mut db: Connection<Db>,
    form: Form<NewBoardForm>,
) -> Result<Redirect, Flash<Redirect>> {
    let form = form.into_inner();
    let is_template = form.is_template.unwrap_or(false);

    // Handle mode
    let template_id = match form.mode.as_str() {
        "template" => form.template_id,
        _ => None,
    };

    match create_board(&mut db, form.name, is_template, template_id).await {
        Ok(new_id) => Err(Flash::success(
            Redirect::to(format!("/boards/{}", new_id)),
            "Board created!",
        )),
        Err(err) => {
            eprintln!("Error creating board: {err}");
            Err(Flash::error(
                Redirect::to("/boards/new"),
                "Could not create board",
            ))
        }
    }
}
