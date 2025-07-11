use crate::{api::Authenticated, db_manage::Db};
use maud::{html, Markup};
use rocket::{
    get,
    request::FlashMessage,
    response::{Flash, Redirect},
};
use rocket_db_pools::Connection;

use super::style::{base_flash, render, Page};

pub fn new_code_form() -> Markup {
    html! {
      main class="container" {
        h1 { "Create New Code" }

        form method="post" action="/codes/new" {
          label for="name" { "Name (example: mark_done)" }
          input type="name" id="name" name="name" required;

          label for="capabilities" {
              r#"Capabilities (example: ["SysLog", { "GetAttribute": "Own" } ]"#
          }
          input type="capabilities" id="capabilities"
                name="capabilities" required;

          label for="script" { "Script" }
          textarea id="script" name="script" {};

          button type="submit" class="contrast" { "Create Code" }
        }
      }
    }
}

#[get("/codes/new")]
pub async fn new_code(
    _auth: Authenticated,
    flash: Option<FlashMessage<'_>>,
) -> Result<Markup, Flash<Redirect>> {
    let contents = new_code_form();

    let page = Page {
        title: html! { title { "Create New Code" } },
        flash: base_flash(flash),
        contents,
    };

    Ok(render(page))
}
