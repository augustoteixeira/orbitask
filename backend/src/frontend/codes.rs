use std::collections::HashMap;

use maud::{html, Markup};

use crate::api::codes::{Action, FormContainer, FormType};
use crate::api::notes::rocket_uri_macro_execute_action;

pub fn render_forms(
    note_id: i64,
    forms: HashMap<String, FormContainer>,
) -> Markup {
    html! {
        @for a in &forms {
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
    };
    html! {
        (form)
    }
}
