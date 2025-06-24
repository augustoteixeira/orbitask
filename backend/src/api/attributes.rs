use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::{get, post};
use rocket_db_pools::Connection;

use crate::db_manage::{self, attributes::Attribute, codes::Code, logs::Log};
use crate::Db;

#[derive(FromForm)]
pub struct NewAttributeForm {
    pub note_id: i64,
    pub key: String,
    pub value: String,
}

#[post("/attributes", data = "<form>")]
pub async fn create_attribute_submit(
    mut db: Connection<Db>,
    form: Form<NewAttributeForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let NewAttributeForm {
        note_id,
        key,
        value,
    } = form.into_inner();

    match db_manage::create_attribute(&mut db, note_id, key, value).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(format!("/notes/{}", note_id)),
            "Attribute created.",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(format!("/notes/{}", note_id)),
            "Failed to create attribute.",
        )),
    }
}
