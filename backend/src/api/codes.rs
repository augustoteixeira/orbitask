use crate::db_manage::codes::{create_code, edit_code};
use crate::frontend::codes::rocket_uri_macro_view_code;
use chrono::NaiveDate;
use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;

use crate::api::Authenticated;
use crate::Db;
use rocket::uri;
use rocket::{post, FromForm};
use serde::{Deserialize, Serialize, Serializer};

#[derive(FromForm)]
pub struct NewCodeForm {
    pub name: String,
    pub capabilities: String,
    pub script: String,
}

#[post("/codes/new", data = "<form>")]
pub async fn create_code_submit(
    _auth: Authenticated,
    mut db: Connection<Db>,
    form: Form<NewCodeForm>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let NewCodeForm {
        name,
        capabilities,
        script,
    } = form.into_inner();

    let note: Option<String> = None;
    match create_code(&mut db, name, capabilities, script).await {
        Ok(name) => Ok(Flash::success(
            Redirect::to(uri!(view_code(name = name, note = note))), // TODO insert a redirect?
            "Code created.",
        )),
        Err(e) => Err(Flash::error(
            Redirect::to("/codes/new"), // TODO use uri! macro
            format!("Failed to create code: {e}"),
        )),
    }
}

#[derive(FromForm)]
pub struct EditCodeForm {
    pub name: String, // Code to be updated
    pub capabilities: String,
    pub script: String,
}

#[post("/codes/edit?<next>", data = "<form>")]
pub async fn edit_code_submit(
    _auth: Authenticated,
    mut db: Connection<Db>,
    form: Form<EditCodeForm>,
    next: Option<String>,
) -> Result<Flash<Redirect>, Flash<Redirect>> {
    let EditCodeForm {
        name,
        capabilities,
        script,
    } = form.into_inner();
    match edit_code(&mut db, &name, &capabilities, &script).await {
        Ok(_) => match next {
            None => Ok(Flash::success(Redirect::to("/"), "Code updated.")),
            Some(s) => Ok(Flash::success(Redirect::to(s), "Code updated.")),
        },
        Err(e) => Err(Flash::error(
            Redirect::to(uri!("/")), // Optionally redirect elsewhere
            format!("Failed to update code: {e}"),
        )),
    }
}

// struct StructType {
//     fields: Vec<Action>
// }

// struct EnumType {
//     variants: Vec<Action>
// }

#[derive(Debug, Serialize, Deserialize)]
pub enum FormType {
    UInt,
    Date,
    Empty,
    // Struct(StuctType)
    // Enum(EnumType)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    pub label: String,
    pub title: String,
    pub form_type: FormType,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FormContainer {
    pub title: String,
    pub label: String,
    pub action: Action,
}

#[derive(Debug)]
pub struct Date(pub NaiveDate); //(#[serde(with = "date_format")] NaiveDate);

const FORMAT: &str = "%Y-%m-%d";

impl Serialize for Date {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.0.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }
}

#[derive(Debug, Serialize)]
pub enum Value {
    UInt(u64),
    Date(Date),
    Empty,
}
