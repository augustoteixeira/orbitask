use crate::db_manage::codes::get_all_code_names;
use crate::db_manage::codes::get_forms;
use crate::Db;
use maud::{html, Markup};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;

use crate::api::Authenticated;
use crate::db_manage::{get_child_notes, get_note, get_root_notes, Note};
use crate::frontend::codes::render_forms;
use crate::frontend::style::{base_flash, render, Page};

pub fn notes_grid(notes: Vec<Note>) -> Markup {
    html! {
      section style=r#"
        display: grid; gap: 1rem;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      "# {
        @for note in notes {
          article style=r#"
            padding: 1rem; border: 1px solid var(--muted-border);
            border-radius: 0.5rem; margin: 0.5rem;
          "# {
              a href={(format!("/notes/{}", note.id))} {
                (note.title)
              }
              p style="font-size: 0.8em; color: var(--muted-color);" {
                (note.description)
              }
              @if let Some(code) = &note.code_name {
                p style="font-size: 0.75em; color: var(--muted-color); font-style: italic;" {
                  "Script: " (code)
                }
              }
          }
        }
      }
    }
}

#[get("/")]
pub async fn root_notes(
    flash: Option<FlashMessage<'_>>,
    _auth: Authenticated,
    mut db: Connection<Db>,
) -> Result<Markup, Flash<Redirect>> {
    let notes = get_root_notes(&mut db).await.unwrap_or_default();
    let contents = html! {
      main {
        section class="main" {
          h2 { "Home" }
          (notes_grid(notes))
          a href="/notes/new" role="button" { "Create New Root Note" }
        }
      }
    };
    let page = Page {
        title: html! {title {"Notes"}},
        flash: base_flash(flash),
        contents,
    };
    Ok(render(page))
}

#[get("/notes/<id>")]
pub async fn show_note(
    _auth: Authenticated,
    id: i64,
    flash: Option<FlashMessage<'_>>,
    mut db: Connection<Db>,
) -> Result<Markup, Flash<Redirect>> {
    let note = match get_note(&mut db, id).await {
        Ok(Some(note)) => note,
        Ok(None) => {
            return Err(Flash::error(Redirect::to("/notes"), "Note not found."))
        }
        Err(_) => {
            return Err(Flash::error(
                Redirect::to("/notes"),
                "Failed to load note.",
            ))
        }
    };

    let child_notes = get_child_notes(&mut db, id).await.unwrap();

    let rendered_children = notes_grid(child_notes);

    let attributes: Vec<(String, String)> = Vec::new();

    let logs: Vec<String> = Vec::new();

    let forms = get_forms(&mut db, note.id).await;

    let contents = html! {
      main class="container" {
        a href="/" { "← Back to Notes" }
        @if let Some(id) = note.parent_id {
          br;
          a href=(uri!(show_note(id))) { "← Back to Parent" }
        }
        h3 { (note.title) }
        p style="color: var(--muted-color); font-size: 0.9em;" {
            "Code: " (note.code_name.unwrap_or("NONE".to_string()))
        }
        @for a in attributes {
            p style="color: var(--muted-color); font-size: 0.9em;" {
                (a.0) ":" (a.1)
            }
        }
        a href={(uri!(new_note(parent_id = Some(note.id))))} role="button" {
          "Create Subnote"
        }
        a href={(uri!(edit_note(note.id)))} role="button" {
          "Edit Note"
        }
        article style=r#"
          padding: 1rem; border: 1px solid var(--muted-border);
          border-radius: 0.5rem; margin: 0.5rem;
        "# {
          p { (note.description) }
        }
        p { (render_forms(note.id, forms))}
        (rendered_children);
        @for l in logs {
            p style="color: var(--muted-color); font-size: 0.9em;" { (l) }
        }
      }
    };

    let page = Page {
        title: html! { title { (note.title) } },
        flash: base_flash(flash),
        contents,
    };
    Ok(render(page))
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

#[get("/notes/<id>/edit")]
pub async fn edit_note(
    _auth: Authenticated,
    id: i64,
    mut db: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Markup, Flash<Redirect>> {
    Ok(html! { h1 {"Unimplemented"} })
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
