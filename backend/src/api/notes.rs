use rocket::form::Form;
use rocket::post;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use std::collections::HashMap;

use crate::api::codes::{parse_fields, NewCodeForm};
use crate::api::Authenticated;
use crate::db_manage::{create_note, Db};

use super::codes::get_form_type;

#[derive(FromForm)]
pub struct CreateNoteForm {
    pub title: String,
    pub description: String,
    pub parent_id: Option<i64>,
    pub code_name: Option<String>,
}

#[post("/notes/new", data = "<form>")]
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

    // TODO: this looks hacky
    let code_name = match code_name {
        None => None,
        Some(s) => match s.as_str() {
            "__none__" => None,
            s => Some(s.to_string()),
        },
    };

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
        Err(e) => Err(Flash::error(
            Redirect::to("/"),
            format!("Failed to create note: {e}."),
        )),
    }
}

#[derive(FromForm, Debug)]
pub struct ExecuteForm {
    pub action_name: String,
    //#[field(name = uncaptured)] // collect all other fields
    #[field(name = "fields")]
    pub fields: HashMap<String, String>,
}

#[post("/notes/<id>/execute", data = "<form>")]
pub async fn execute_action(
    _auth: Authenticated,
    mut db: Connection<Db>,
    id: i64,
    form: Form<ExecuteForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    println!("{form:?}");
    let form_type = get_form_type(id, form.action_name.clone());
    let value = parse_fields(&form_type, &form.fields, "".to_string());
    Err(Flash::error(
        Redirect::to("/"),
        format!("execute not yet implemented: {value:?}"),
    ))
}
