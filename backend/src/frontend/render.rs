use std::collections::HashMap;

use crate::api::codes::{Action, FormContainer, FormType};
use crate::api::notes::rocket_uri_macro_execute_action;
use crate::db_manage::Note;
use crate::frontend::codes::rocket_uri_macro_view_code;
use crate::frontend::notes::rocket_uri_macro_edit_note;
use crate::frontend::notes::rocket_uri_macro_new_note;
use crate::frontend::notes::rocket_uri_macro_show_note;
use crate::frontend::view::render_notes_grid;
use markdown;
use maud::{html, Markup, PreEscaped};
use rocket::uri;

pub fn render_note(
    note: &Note,
    attributes: &Vec<(String, String)>,
    forms: &HashMap<String, FormContainer>,
    child_notes: &Vec<Note>,
    ancestors: &Vec<(i64, String)>,
    logs: &Vec<String>,
) -> Markup {
    let rendered_children = render_notes_grid(child_notes);
    html! {
          main {
            nav class="breadcrumb" {
              a href="/" { "Home" }
              @for (id, title) in ancestors {
                span class="crumb" { " / " }
                a href=(uri!(show_note(*id))) { (title) }
              }
              span class="crumb" { " / " }
              span { (note.title.clone()) }
            }

            div class="note-header" {
              h2  { (note.title) }
              @if let Some(code_name) = note.code_name.clone() {
                a href={(uri!(view_code(name=code_name.clone(),
                                        note=Some(note.id.to_string()))))}
                  role="button" {
                  (code_name.clone())
                }
              }
            }
            div class="attribute-container" {
              @for a in attributes {
                  p class="badge attribute" {
                      (a.0) ": " (a.1)
                  }
              }
            }
            article { p { (PreEscaped(markdown::to_html(&note.description))) } }

            (render_forms(note.id, forms))

            div class="note-bottom-buttons" {
              a href={(uri!(new_note(parent_id = Some(note.id))))} role="button" {
                "Create Subnote"
              }
              a href={(uri!(edit_note(note.id)))} role="button" {
                "Edit Note"
              }
              a href=(uri!(crate::frontend::notes::delete_note_confirm(note.id))) role="button" {
                "Delete Note"
              }
            }
            h3 {"Subnotes"}
            (rendered_children);
            @for l in logs {
                p style="color: var(--muted-color); font-size: 0.9em;" { (l) }
            }
          }
    }
}

pub fn render_forms(
    note_id: i64,
    forms: &HashMap<String, FormContainer>,
) -> Markup {
    html! {
        @for a in forms {
            h5 { (a.1.title) }
            (render_form(note_id, &a.1.action, a.0.to_string()))
        }
    }
}

pub fn render_form(note_id: i64, action: &Action, prefix: String) -> Markup {
    let form = match action.form_type {
        FormType::UInt => html! {
          form method="post" action=(uri!(execute_action(note_id))) {
            div class="field" {
              input type="hidden" name="action_label" value=(prefix);

              label for=(format!("fields[{}]",action.label)) { (action.title) }
              input type="int" name=(format!("fields[{}]",action.label));
            }

            button type="submit" { "Execute" }
          }
        },
        FormType::Date => html! {
          form method="post" action=(uri!(execute_action(note_id))) {
            div class="field" {
              input type="hidden" name="action_label" value=(prefix);

              label for=(format!("fields[{}]",action.label)) { (action.title) }
              input type="date" name=(format!("fields[{}]",action.label));

              button type="submit" { "Execute" }
            }
          }
        },
        FormType::Empty => html! {
          form method="post" action=(uri!(execute_action(note_id))) {
            input type="hidden" name="action_label" value=(prefix);

            button type="submit" { "Execute" }
          }
        },
    };
    html! {
        (form)
    }
}
