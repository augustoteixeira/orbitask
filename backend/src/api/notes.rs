use rocket::form::Form;
use rocket::post;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use std::collections::HashMap;

use crate::api::Authenticated;
use crate::db_manage::codes::{execute, get_forms, parse_fields};
use crate::db_manage::{create_note, Db};
use crate::frontend::notes::rocket_uri_macro_show_note;

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
    pub action_label: String,
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
    let forms = get_forms(&mut db, id).await.map_err(|e| {
        Flash::error(
            Redirect::to("/"),
            format!("could not get forms: note {:?} {e}", &id),
        )
    })?;
    let form_container = forms.get(&form.action_label).ok_or(Flash::error(
        Redirect::to(uri!(show_note(id))),
        format!("action not found: {}", form.action_label),
    ))?;
    let action = &form_container.action;
    let value =
        parse_fields(&action.form_type, &form.fields, &action.label)
            .map_err(|e| {
                Flash::error(
                    Redirect::to("/"), format!(
                        "parsing failed: form_type {:?}, form.fields {:?}, prefix {:?}{e}",
                      &action.form_type,
                      &form.fields,
                      "".to_string()
                    )
                )
            })?;
    let message = execute(&mut db, id, &action, &value).await.map_err(|e| {
        Flash::error(
            Redirect::to("/"),
            format!("executing failed: id {id:?}, action {action:?}, value {:?}\n{e}", &value),
        )
    })?;
    Ok(Flash::success(
        Redirect::to(uri!(show_note(id))),
        format!("Code correctly executed: {message}"),
    ))
}
