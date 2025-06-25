use crate::Db;
use maud::{html, Markup};
use rocket::http::uri;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;

use crate::api::Authenticated;
use crate::db_manage::{get_child_notes, get_note, Note};
use crate::frontend::style::{base_flash, render, Page};
use crate::sqlx::FromRow;

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

// #[get("/notes")]
// pub async fn notes_page(
//     flash: Option<FlashMessage<'_>>,
//     _auth: Authenticated,
//     mut db: Connection<Db>,
// ) -> Result<Markup, Flash<Redirect>> {
//     let notes = get_all_notes(&mut db).await.unwrap_or_default();
//     let contents = html! {
//       main {
//         section class="main" {
//           h2 { "Your Notes" }
//           (notes_grid(notes))
//           a href="/notes/new" role="button" { "Create New Note" }
//         }
//       }
//     };
//     let page = Page {
//         title: html! {title {"Notes"}},
//         flash: base_flash(flash),
//         contents,
//     };
//     Ok(render(page))
// }

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

    let contents = html! {
        main class="container" {
            h1 { (note.title) }
            p style="color: var(--muted-color); font-size: 0.9em;" {
                "Code: " (note.code_name.unwrap_or("NONE".to_string()))
            }
            p { (note.description) }
            a href="/notes" { "← Back to Notes" }
            (rendered_children);
        }
    };
    //let contents = html! { main { hi { "A" } } };

    let page = Page {
        title: html! { title { (note.title) } },
        flash: base_flash(flash),
        contents,
    };
    Ok(render(page))
}
