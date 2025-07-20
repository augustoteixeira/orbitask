use crate::db_manage::attributes::get_attributes;
use crate::db_manage::codes::get_forms;
use crate::db_manage::notes::{get_ancestors, update_note};
use crate::db_manage::Db;
use maud::{html, Markup};
use rocket::get;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket::uri;
use rocket_db_pools::Connection;

use crate::api::notes::rocket_uri_macro_delete_attribute_submit;
use crate::api::notes::rocket_uri_macro_edit_note_submit;
use crate::api::notes::rocket_uri_macro_update_or_add_attribute_submit;
use crate::api::Authenticated;
use crate::db_manage::codes::get_all_code_names;
use crate::db_manage::{get_child_notes, get_note, get_root_notes};
use crate::frontend::render::render_note;
use crate::frontend::style::{base_flash, render, Page};
use crate::frontend::view::render_notes_grid;

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

pub fn new_note_form(codes: Vec<String>, parent_id: Option<i64>) -> Markup {
    html! {
      main class="container" {
        h1 { "Create New Note" }

        form method="post" action="/notes/new" {
          label for="title" { "Title" }
          input type="text" id="title" name="title" required;

          label for="description" { "Description (Markdown)" }
          textarea id="description" name="description" {};

          label for="code_name" { "Behavior (Code)" }
          fieldset {
            legend { "Code" }
            // Option for "no code"
            label {
              input type="radio" name="code_name" value="__none__" required checked;
              " No code"
            }
            // Options for each code name
            @for code in codes.iter() {
              label {
                input type="radio" name="code_name" value=(code);
                (code)
              }
            }
          }

          @if let Some(pid) = parent_id {
            input type="hidden" name="parent_id" value=(pid);
          }

          button type="submit" class="contrast" { "Create Notes" }
        }
      }
    }
}

#[get("/notes/new?<parent_id>")]
pub async fn new_note(
    _auth: Authenticated,
    parent_id: Option<i64>,
    mut db: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Markup, Flash<Redirect>> {
    let codes = get_all_code_names(&mut db).await.map_err(|e| {
        Flash::error(
            Redirect::to(uri!(root_notes)),
            format!("Failed to load code list: {e}."),
        )
    })?;
    let contents = new_note_form(codes, parent_id);

    let page = Page {
        title: html! { title { "Create New Note" } },
        flash: base_flash(flash),
        contents,
    };

    Ok(render(page))
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
) -> Result<Markup, Flash<Redirect>> {
    let note = match crate::db_manage::notes::get_note(&mut db, id).await {
        Ok(Some(note)) => note,
        _ => return Err(Flash::error(Redirect::to("/"), "Note not found.")),
    };

    let contents = html! {
        main class="container" {
            h1 { "Confirm Delete Note" }
            p { "Are you sure you want to delete the note: " (note.title) "?" }
            form method="post" action=(uri!(crate::api::notes::delete_note_submit(id))) {
                button type="submit" { "Yes, delete" }
            }
            a href=(uri!(show_note(id))) { "Cancel" }
        }
    };

    let page = crate::frontend::style::Page {
        title: html! { title { "Confirm Delete" } },
        flash: crate::frontend::style::base_flash(flash),
        contents,
    };

    Ok(render(page))
}
