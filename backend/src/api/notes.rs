use rocket::form::Form;
use rocket::post;
use rocket::response::{Flash, Redirect};
use rocket::{uri, FromForm};
use rocket_db_pools::Connection;
use sqlx::Acquire;
use std::collections::HashMap;

use crate::api::Authenticated;
use crate::db_manage::attributes::{delete_attribute, set_attribute};
use crate::db_manage::codes::{execute, get_forms, parse_fields};
use crate::db_manage::notes::update_note;
use crate::db_manage::{create_note, Db};
use crate::frontend::notes::rocket_uri_macro_edit_note;
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
    let mut tx = db.begin().await.map_err(|e| {
        Flash::error(Redirect::to("/"), format!("Cannot begin tx: {e}."))
    })?;
    let note_id =
        create_note(&mut *tx, parent_id, title, description, code_name)
            .await
            .map_err(|e| {
                Flash::error(
                    Redirect::to("/"),
                    format!("Failed to create note: {e}."),
                )
            })?;
    match tx.commit().await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!(show_note(note_id))),
            "Note created successfully.",
        )),
        Err(e) => Err(Flash::error(
            Redirect::to("/"),
            format!("Cannot commit tx: {e}."),
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
    let mut tx = db.begin().await.map_err(|e| {
        Flash::error(Redirect::to("/"), format!("Cannot begin tx: {e}."))
    })?;
    let message = execute(&mut *tx, id, &form_container, &value).await.map_err(|e| {
        Flash::error(
            Redirect::to("/"),
            format!("executing failed: id {id:?}, form {form_container:?}, value {:?}\n{e}", &value),
        )
    })?;
    match tx.commit().await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!(show_note(id))),
            format!("Code correctly executed: {message}."),
        )),
        Err(e) => Err(Flash::error(
            Redirect::to("/"),
            format!("Cannot commit tx: {e}."),
        )),
    }
}

#[derive(FromForm)]
pub struct EditNoteForm {
    pub title: String,
    pub description: String,
    pub code_name: Option<String>,
    pub attributes: Option<HashMap<String, String>>,
}

#[post("/notes/<id>/edit", data = "<form>")]
pub async fn edit_note_submit(
    _auth: Authenticated,
    mut db: Connection<Db>,
    id: i64,
    form: Form<EditNoteForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let EditNoteForm {
        title,
        description,
        code_name,
        attributes,
    } = form.into_inner();

    let code_name = match code_name {
        Some(s) if s.trim().is_empty() => None,
        other => other,
    };

    if let Err(e) =
        update_note(&mut db, id, title, description, code_name).await
    {
        return Err(Flash::error(
            Redirect::to(uri!(crate::frontend::notes::edit_note(id))),
            format!("Failed to update note: {e}"),
        ));
    }

    if let Some(map) = attributes {
        for (key, value) in map {
            if let Err(e) = set_attribute(&mut db, id, &key, &value).await {
                return Err(Flash::error(
                    Redirect::to(uri!(edit_note(id))),
                    format!("Failed to update attribute: {e}"),
                ));
            }
        }
    }
    Ok(Flash::success(
        Redirect::to(uri!(crate::frontend::notes::show_note(id))),
        "Note updated.",
    ))
}

#[post("/notes/<id>/attributes/<key>/delete")]
pub async fn delete_attribute_submit(
    id: i64,
    key: &str,
    _auth: crate::api::Authenticated,
    mut db: Connection<Db>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    match delete_attribute(&mut db, id, key).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(uri!(crate::frontend::notes::edit_note(id))),
            format!("Attribute '{key}' deleted."),
        )),
        Err(e) => Err(Flash::error(
            Redirect::to(uri!(crate::frontend::notes::edit_note(id))),
            format!("Failed to delete attribute '{key}': {e}"),
        )),
    }
}

#[derive(FromForm)]
pub struct AttributeForm {
    pub key: String,
    pub value: String,
}

#[post("/notes/<id>/attributes/add", data = "<form>")]
pub async fn update_or_add_attribute_submit(
    id: i64,
    _auth: crate::api::Authenticated,
    mut db: Connection<Db>,
    form: Form<AttributeForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let AttributeForm { key, value } = form.into_inner();

    set_attribute(&mut db, id, &key, &value)
        .await
        .map_err(|e| {
            Flash::error(
                Redirect::to(uri!(crate::frontend::notes::edit_note(id))),
                format!("Failed to set attribute: {e}"),
            )
        })?;

    Ok(Flash::success(
        Redirect::to(uri!(crate::frontend::notes::edit_note(id))),
        "Attribute added or updated.",
    ))
}

#[post("/notes/<id>/delete")]
pub async fn delete_note_submit(
    _auth: crate::api::Authenticated,
    mut db: Connection<Db>,
    id: i64,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    // Fetch the note to determine the parent redirect before deletion
    let parent_redirect =
        match crate::db_manage::notes::get_note(&mut db, id).await {
            Ok(Some(note)) => {
                if let Some(pid) = note.parent_id {
                    Redirect::to(uri!(crate::frontend::notes::show_note(pid)))
                } else {
                    Redirect::to(uri!(crate::frontend::notes::root_notes))
                }
            }
            _ => Redirect::to(uri!(crate::frontend::notes::root_notes)),
        };

    match crate::db_manage::notes::delete_note(&mut db, id).await {
        Ok(_) => Ok(Flash::success(
            parent_redirect,
            "Note deleted successfully.",
        )),
        Err(e) => Err(Flash::error(
            Redirect::to(uri!(crate::frontend::notes::edit_note(id))),
            format!("Failed to delete note: {e}"),
        )),
    }
}
