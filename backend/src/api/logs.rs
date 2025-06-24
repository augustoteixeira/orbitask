use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket::{get, post};
use rocket_db_pools::Connection;

use crate::db_manage::{self, attributes::Attribute, codes::Code, logs::Log};
use crate::Db;

#[derive(FromForm)]
pub struct NewLogForm {
    pub note_id: i64,
    pub kind: String,
    pub message: String,
    // no file upload for now
}

#[post("/logs", data = "<form>")]
pub async fn create_log_submit(
    mut db: Connection<Db>,
    form: Form<NewLogForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let NewLogForm {
        note_id,
        kind,
        message,
    } = form.into_inner();

    match db_manage::create_log(&mut db, note_id, kind, message, None).await {
        Ok(_) => Ok(Flash::success(
            Redirect::to(format!("/notes/{}", note_id)),
            "Log entry added.",
        )),
        Err(_) => Err(Flash::error(
            Redirect::to(format!("/notes/{}", note_id)),
            "Failed to add log entry.",
        )),
    }
}
