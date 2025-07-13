use std::collections::HashMap;

use crate::api::codes::{Action, FormContainer, FormType};
use crate::api::notes::rocket_uri_macro_execute_action;
use crate::db_manage::Note;
use crate::frontend::codes::rocket_uri_macro_view_code;
use crate::frontend::notes::rocket_uri_macro_edit_note;
use crate::frontend::notes::rocket_uri_macro_new_note;
use crate::frontend::notes::rocket_uri_macro_show_note;
use maud::{html, Markup};
use rocket::uri;

pub fn render_notes_grid(notes: &Vec<Note>) -> Markup {
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
            p style="font-size: 0.8em; color: var(--muted-color); margin-bottom: 0.2rem" {
              (note.description)
            }
            @if let Some(code) = &note.code_name {
              p style="font-size: 0.75em; color: var(--muted-color); font-style: italic; margin-bottom: 0.2rem" {
                "Script: " (code)
              }
            }
          }
        }
      }
    }
}

pub fn render_note(
    note: &Note,
    attributes: &Vec<(String, String)>,
    forms: &HashMap<String, FormContainer>,
    child_notes: &Vec<Note>,
    logs: &Vec<String>,
) -> Markup {
    let rendered_children = render_notes_grid(child_notes);
    html! {
          main class="container" {
            a href="/" { "← Back to Notes" }
            @if let Some(id) = note.parent_id {
              br;
              a href=(uri!(show_note(id))) { "← Back to Parent" }
            }
            h3 style="margin-bottom: 1rem; margin-top: 1rem;" { (note.title) }
            //p style="color: var(--muted-color); font-size: 0.9em; margin-bottom: 0.5rem" {
            @if let Some(code_name) = note.code_name.clone() {
              a href={(uri!(view_code(code_name.clone())))} role="button" {
                "Code: " (code_name.clone())
              }
            }
            @for a in attributes {
                p style="color: var(--muted-color); font-size: 0.9em; margin-bottom: 0.5rem" {
                    (a.0) ":" (a.1)
                }
            }
            article style=r#"
          padding: 1rem; border: 1px solid var(--muted-border);
          border-radius: 0.5rem; margin: 0.5rem; padding: 0.5rem
        "# {
              p style="margin-bottom: 0.2rem" { (note.description) }
            }
            (render_forms(note.id, forms))
            a href={(uri!(new_note(parent_id = Some(note.id))))} role="button" {
              "Create Subnote"
            }
            a href={(uri!(edit_note(note.id)))} role="button" {
              "Edit Note"
            }
            h5 {"Subnotes"}
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
            input type="hidden" name="action_label" value=(prefix);

            label for=(format!("fields[{}]",action.label)) { (action.title) }
            input type="int" name=(format!("fields[{}]",action.label));

            button type="submit" { "Execute" }
          }
        },
        FormType::Date => html! {
          form method="post" action=(uri!(execute_action(note_id))) {
            input type="hidden" name="action_label" value=(prefix);

            label for=(format!("fields[{}]",action.label)) { (action.title) }
            input type="date" name=(format!("fields[{}]",action.label));

            button type="submit" { "Execute" }
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
