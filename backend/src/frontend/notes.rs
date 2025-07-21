use crate::db_manage::attributes::get_attributes;
use crate::db_manage::codes::get_forms;
use crate::db_manage::notes::get_ancestors;
use crate::db_manage::Db;
use rocket::get;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::uri;
use rocket_db_pools::Connection;

use crate::api::Authenticated;
use crate::db_manage::codes::get_all_code_names;
use crate::db_manage::{get_child_notes, get_note, get_root_notes};

use super::view::{MyFlash, View, ViewState};

#[get("/")]
pub async fn root_notes(
    flash: Option<FlashMessage<'_>>,
    _auth: Authenticated,
    mut db: Connection<Db>,
) -> View {
    let notes = get_root_notes(&mut db).await.unwrap_or_default();
    return View {
        state: ViewState::Root(notes),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    };
}

#[get("/notes/<id>")]
pub async fn show_note(
    _auth: Authenticated,
    id: i64,
    flash: Option<FlashMessage<'_>>,
    mut db: Connection<Db>,
) -> Result<View, Flash<Redirect>> {
    let note = match get_note(&mut db, id).await {
        Ok(Some(note)) => note,
        Ok(None) => {
            return Err(Flash::error(Redirect::to("/"), "Note not found."))
        }
        Err(e) => {
            return Err(Flash::error(
                Redirect::to("/"),
                format!("Failed to load note: {e}"),
            ))
        }
    };

    let child_notes = get_child_notes(&mut db, id).await.unwrap();

    let attributes: Vec<(String, String)> =
        get_attributes(&mut db, id).await.unwrap();

    let logs: Vec<String> = Vec::new();

    let forms = get_forms(&mut db, note.id).await.map_err(|e| {
        Flash::error(Redirect::to("/"), format!("Failed to load forms: {e}"))
    })?;

    let ancestors = get_ancestors(&mut db, id).await.map_err(|e| {
        Flash::error(Redirect::to("/"), format!("Failed to get ancestors: {e}"))
    })?;

    Ok(View {
        state: ViewState::Note(
            note,
            attributes,
            forms,
            child_notes,
            ancestors,
            logs,
        ),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    })
}

#[get("/notes/new?<parent_id>")]
pub async fn new_note(
    _auth: Authenticated,
    parent_id: Option<i64>,
    mut db: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Result<View, Flash<Redirect>> {
    let codes = get_all_code_names(&mut db).await.map_err(|e| {
        Flash::error(
            Redirect::to(uri!(root_notes)),
            format!("Failed to load code list: {e}."),
        )
    })?;

    Ok(View {
        state: ViewState::NoteNew(codes, parent_id),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    })
}

#[get("/notes/<id>/edit")]
pub async fn edit_note(
    _auth: Authenticated,
    mut db: Connection<Db>,
    id: i64,
    flash: Option<FlashMessage<'_>>,
) -> Result<View, Flash<Redirect>> {
    let note = get_note(&mut db, id)
        .await
        .map_err(|e| {
            Flash::error(Redirect::to(uri!(root_notes)), format!("Error: {e}"))
        })?
        .ok_or_else(|| {
            Flash::error(Redirect::to(uri!(root_notes)), "Note not found")
        })?;

    let attributes = get_attributes(&mut db, id).await.unwrap_or_default();

    let codes = get_all_code_names(&mut db).await.map_err(|e| {
        Flash::error(Redirect::to(uri!(root_notes)), format!("Error: {e}"))
    })?;

    Ok(View {
        state: ViewState::NoteEdit(id, note, codes, attributes),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    })
}

#[get("/notes/<id>/delete/confirm")]
pub async fn delete_note_confirm(
    _auth: Authenticated,
    id: i64,
    flash: Option<FlashMessage<'_>>,
    mut db: Connection<Db>,
) -> Result<View, Flash<Redirect>> {
    let note = match crate::db_manage::notes::get_note(&mut db, id).await {
        Ok(Some(note)) => note,
        _ => return Err(Flash::error(Redirect::to("/"), "Note not found.")),
    };
    Ok(View {
        state: ViewState::NoteConfirmDelete(id, note.title),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    })
}
