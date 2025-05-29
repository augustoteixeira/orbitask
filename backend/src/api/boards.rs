use crate::db_manage::boards::create_board;
use crate::db_manage::notes::create_note;
use crate::db_manage::states::create_state;
use crate::Db;

use crate::api::Authenticated;
use crate::frontend::board::rocket_uri_macro_board;
use crate::frontend::board::rocket_uri_macro_board_settings;
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
            Redirect::to(uri!(board(new_id))),
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

#[derive(FromForm)]
pub struct CreateStateForm {
    pub name: String,
    pub is_finished: bool,
}

#[post("/boards/<id>/create_state", data = "<form>")]
pub async fn create_state_submit(
    _auth: Authenticated,
    mut db: Connection<Db>,
    id: i64,
    form: Form<CreateStateForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let CreateStateForm { name, is_finished } = form.into_inner();

    // Empty names are not allowed
    if name.trim().is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(board_settings(id))),
            "State name cannot be empty.",
        ));
    }

    match create_state(&mut db, id, name, is_finished).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!(board_settings(id))),
            "State created successfully.",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(uri!(board_settings(id))),
            "Failed to create state.",
        )),
    }
}

#[derive(FromForm)]
pub struct CreateNoteForm {
    pub name: String,
    pub start_date: String,
    pub due_date: String,
    pub template_id: Option<i64>,
}

#[post("/boards/<id>/states/<state_id>/notes", data = "<form>")]
pub async fn create_note_submit(
    _auth: Authenticated,
    mut db: Connection<Db>,
    id: i64,
    state_id: i64,
    form: Form<CreateNoteForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let CreateNoteForm {
        name,
        start_date,
        due_date,
        template_id,
    } = form.into_inner();

    // Empty names are not allowed
    if name.trim().is_empty() {
        return Err(Flash::error(
            Redirect::to(uri!(board(id))),
            "Note name cannot be empty.",
        ));
    }

    match create_note(
        &mut db,
        id,
        state_id,
        name,
        start_date,
        due_date,
        template_id,
    )
    .await
    {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!(board(id))),
            "Note created successfully.",
        )),
        Err(e) => Err(Flash::error(
            Redirect::to(uri!(board(id))),
            "Failed to create note.",
        )),
    }
}
