use crate::api::codes::rocket_uri_macro_edit_code_submit;
use crate::frontend::notes::rocket_uri_macro_root_notes;
use crate::frontend::notes::rocket_uri_macro_show_note;
use crate::{
    api::Authenticated,
    db_manage::{codes::Code, Db},
};
use maud::{html, Markup};
use rocket::uri;
use rocket::{
    get,
    request::FlashMessage,
    response::{Flash, Redirect},
};
use rocket_db_pools::Connection;

use super::style::{base_flash, render, Page};
use crate::frontend::view::{MyFlash, View, ViewState};

#[get("/codes/<name>?<note>")]
pub async fn view_code(
    _auth: Authenticated,
    mut db: Connection<Db>,
    name: String,
    note: Option<String>,
    flash: Option<FlashMessage<'_>>,
) -> Result<View, Flash<Redirect>> {
    use crate::db_manage::codes::get_code_by_name;

    let code = get_code_by_name(&mut db, &name)
        .await
        .map_err(|e| {
            Flash::error(
                Redirect::to(uri!(root_notes)),
                format!("DB error: {e}"),
            )
        })?
        .ok_or_else(|| Flash::error(Redirect::to("/"), "Code not found"))?;

    let next: Option<String> =
        note.map(|id| uri!(show_note(id.parse::<i64>().unwrap())).to_string());

    Ok(View {
        state: ViewState::Code(code, next),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    })
}

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

pub fn edit_code_form(code: &Code, next_url: Option<String>) -> Markup {
    html! {
      main class="container" {
        h1 { "Edit Code" }

        form method="post"
             action=(uri!(edit_code_submit(next=next_url))) {
          input type="hidden" name="name" value=(code.name);

          label for="capabilities" {
              r#"Capabilities (example: ["SysLog", { "GetAttribute": "Own" } ])"#
          }
          input type="capabilities" id="capabilities"
                name="capabilities" required value=(code.capabilities);

          label for="script" { "Script" }
          textarea id="script" name="script" rows="50" {
              (code.script)
          }

          button type="submit" class="contrast" { "Update Code" }
        }
      }
    }
}

#[get("/codes/<name>/edit?<next>")]
pub async fn edit_code(
    _auth: Authenticated,
    mut db: Connection<Db>,
    name: String,
    next: Option<String>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Markup, Flash<Redirect>> {
    use crate::db_manage::codes::get_code_by_name; // You may need a simple `get_code_by_name`

    let code = get_code_by_name(&mut db, &name)
        .await
        .map_err(|e| Flash::error(Redirect::to("/"), format!("DB error: {e}")))?
        .ok_or_else(|| Flash::error(Redirect::to("/"), "Code not found"))?;

    let contents = edit_code_form(&code, Some("/notes/1".to_string()));
    let page = Page {
        title: html! { title { "Edit Code" } },
        flash: base_flash(flash),
        contents,
    };
    Ok(render(page))
}

#[get("/codes")]
pub async fn list_codes(
    _auth: Authenticated,
    mut db: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Result<Markup, Flash<Redirect>> {
    let codes = crate::db_manage::codes::get_all_code_names(&mut db)
        .await
        .map_err(|e| {
            Flash::error(Redirect::to("/"), format!("DB error: {e}"))
        })?;

    let no_note = Option::<String>::None;
    let contents = html! {
        main class="container" {
            h1 { "All Codes" }
            ul {
                @for name in codes {
                    li {
                        a href=(uri!(crate::frontend::codes::view_code(name=&name, note=no_note.clone()))) {
                            (name)
                        }
                    }
                }
            }
            nav style="margin-top: 1rem" {
                a href=(uri!(crate::frontend::codes::new_code)) role="button" {
                    "Create New Code"
                }
            }
        }
    };

    let page = Page {
        title: html! { title { "All Codes" } },
        flash: base_flash(flash),
        contents,
    };

    Ok(render(page))
}
