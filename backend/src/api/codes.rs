use rocket::form::Form;
use rocket::post;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;

use crate::api::Authenticated;
use crate::db_manage::{self};
use crate::Db;

#[derive(FromForm)]
pub struct NewCodeForm {
    pub name: String,
    pub code: String,
}

#[post("/codes", data = "<form>")]
pub async fn create_code_submit(
    _auth: Authenticated,
    mut db: Connection<Db>,
    form: Form<NewCodeForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let NewCodeForm { name, code } = form.into_inner();

    match db_manage::create_code(&mut db, name, code).await {
        Ok(_) => Ok(Flash::success(Redirect::to("/codes"), "Code created.")),
        Err(_) => Err(Flash::error(
            Redirect::to("/codes"), // TODO use uri! macro
            "Failed to create code.",
        )),
    }
}
