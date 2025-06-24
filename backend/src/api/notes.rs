use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::{get, post};
use rocket_db_pools::Connection;

use crate::api::Authenticated;
use crate::db_manage::{self, attributes::Attribute, codes::Code, logs::Log};
use crate::db_manage::{create_note, Db};

#[derive(FromForm)]
pub struct CreateNoteForm {
    pub title: String,
    pub description: String,
    pub parent_id: Option<i64>,
    pub code_name: Option<String>,
}

#[post("/notes/create", data = "<form>")]
pub async fn create_note_submit(
    _auth: Authenticated,
    mut db: Connection<Db>,
    form: Form<CreateNoteForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let CreateNoteForm {
        title,
        description,
        parent_id,
        code_name,
    } = form.into_inner();

    if title.trim().is_empty() {
        return Err(Flash::error(
            Redirect::to("/"),
            "Note title cannot be empty.",
        ));
    }

    match create_note(&mut db, parent_id, title, description, code_name).await {
        Ok(note_id) => Ok(Flash::success(
            Redirect::to(format!("/notes/{note_id}")),
            "Note created successfully.",
        )),
        Err(_) => {
            Err(Flash::error(Redirect::to("/"), "Failed to create note."))
        }
    }
}
