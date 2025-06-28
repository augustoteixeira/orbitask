use maud::{html, Markup};

use crate::api::codes::{Action, FormType};
use crate::api::notes::rocket_uri_macro_execute_action;

pub fn form(note_id: i64, action: Action, prefix: String) -> Markup {
    let form = match action.form_type {
        FormType::UInt(field) => html! {
          form method="post" action=(uri!(execute_action(note_id))) {
            input type="hidden" name="action_name" value="delay";

            label for=(format!("fields[{}]",field.label)) { (field.title) }
            input type="int" name=(format!("fields[{}]",field.label));

            button type="submit" { "Execute" }
          }
        },
    };
    html! {
        b { (action.title) }
        (form)
    }
}
