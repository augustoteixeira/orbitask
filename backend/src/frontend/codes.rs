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

#[get("/codes/new")]
pub async fn new_code(
    _auth: Authenticated,
    flash: Option<FlashMessage<'_>>,
) -> Result<View, Flash<Redirect>> {
    Ok(View {
        state: ViewState::CodeNew(),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    })
}

#[get("/codes/<name>/edit?<next>")]
pub async fn edit_code(
    _auth: Authenticated,
    mut db: Connection<Db>,
    name: String,
    next: Option<String>,
    flash: Option<FlashMessage<'_>>,
) -> Result<View, Flash<Redirect>> {
    use crate::db_manage::codes::get_code_by_name; // You may need a simple `get_code_by_name`

    let code = get_code_by_name(&mut db, &name)
        .await
        .map_err(|e| Flash::error(Redirect::to("/"), format!("DB error: {e}")))?
        .ok_or_else(|| Flash::error(Redirect::to("/"), "Code not found"))?;

    Ok(View {
        state: ViewState::CodeEdit(code, next),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    })
}

#[get("/codes")]
pub async fn list_codes(
    _auth: Authenticated,
    mut db: Connection<Db>,
    flash: Option<FlashMessage<'_>>,
) -> Result<View, Flash<Redirect>> {
    let codes = crate::db_manage::codes::get_all_code_names(&mut db)
        .await
        .map_err(|e| {
            Flash::error(Redirect::to("/"), format!("DB error: {e}"))
        })?;

    let no_note = Option::<String>::None;

    Ok(View {
        state: ViewState::CodeList(codes, no_note),
        flash: flash.into_iter().map(MyFlash::from).collect(),
    })
}
