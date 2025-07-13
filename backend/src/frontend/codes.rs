use crate::frontend::notes::rocket_uri_macro_root_notes;
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

#[get("/codes/<name>")]
pub async fn view_code(
    _auth: Authenticated,
    mut db: Connection<Db>,
    name: String,
    flash: Option<FlashMessage<'_>>,
) -> Result<Markup, Flash<Redirect>> {
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

    let contents = html! {
        main class="container" {
            h1 { "Code Details" }

            p { strong { "Name:" } " " (code.name.clone()) }
            p { strong { "Capabilities:" } " " (code.capabilities.clone()) }

            h2 { "Script" }
            pre {
                code { (code.script.clone()) }
            }

            nav style="margin-top: 1rem" {
                a href=(uri!(edit_code(name = code.name.clone()))) role="button" {
                    "Edit Code"
                }
            }
        }
    };

    let page = Page {
        title: html! { title { "View Code" } },
        flash: base_flash(flash),
        contents,
    };

    Ok(render(page))
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

pub fn edit_code_form(code: &Code) -> Markup {
    html! {
      main class="container" {
        h1 { "Edit Code" }

        form method="post" action="/codes/edit" {
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

#[get("/codes/<name>/edit")]
pub async fn edit_code(
    _auth: Authenticated,
    mut db: Connection<Db>,
    name: String,
    flash: Option<FlashMessage<'_>>,
) -> Result<Markup, Flash<Redirect>> {
    use crate::db_manage::codes::get_code_by_name; // You may need a simple `get_code_by_name`

    let code = get_code_by_name(&mut db, &name)
        .await
        .map_err(|e| Flash::error(Redirect::to("/"), format!("DB error: {e}")))?
        .ok_or_else(|| Flash::error(Redirect::to("/"), "Code not found"))?;

    let contents = edit_code_form(&code);
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

    let contents = html! {
        main class="container" {
            h1 { "All Codes" }
            ul {
                @for name in codes {
                    li {
                        a href=(uri!(crate::frontend::codes::view_code(&name))) {
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
