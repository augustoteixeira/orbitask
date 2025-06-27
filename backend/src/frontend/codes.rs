use crate::db_manage::codes::get_all_code_names;
use crate::Db;
use maud::{html, Markup};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;

use crate::api::Authenticated;
use crate::db_manage::codes::{Action, FormType};
use crate::db_manage::{get_child_notes, get_note, get_root_notes, Note};
use crate::frontend::style::{base_flash, render, Page};

pub fn form(note_id: i64, action: Action) -> Markup {
    let form = match action.form_type {
        FormType::UInt(field) => html! {
        form method="post" action="/notes/execute/123" {
          label for="int" { (field.title) }
          input type="text" id=(field.label) name=(field.label) required;
          button type="submit" class="contrast" { "Submit" }
        }    },
    };
    html! {
        b { (action.title) }
        (form)
    }
}
