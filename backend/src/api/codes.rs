use chrono::NaiveDate;
//use rocket::form::Form;
//use rocket::post;
//use rocket::response::{Flash, Redirect};
//use rocket_db_pools::Connection;

//use crate::api::Authenticated;
//use crate::db_manage::{self};
//use crate::Db;
use serde::{Deserialize, Serialize, Serializer};

// #[derive(FromForm)]
// pub struct NewCodeForm {
//     pub name: String,
//     pub capabilities: String,
//     pub script: String,
// }

// #[post("/codes", data = "<form>")]
// pub async fn create_code_submit(
//     _auth: Authenticated,
//     mut db: Connection<Db>,
//     form: Form<NewCodeForm>,
// ) -> Result<Flash<Redirect>, Flash<Redirect>> {
//     let NewCodeForm {
//         name,
//         capabilities,
//         script,
//     } = form.into_inner();

//     match db_manage::create_code(&mut db, name, capabilities, script).await {
//         Ok(_) => Ok(Flash::success(Redirect::to("/codes"), "Code created.")),
//         Err(_) => Err(Flash::error(
//             Redirect::to("/codes"), // TODO use uri! macro
//             "Failed to create code.",
//         )),
//     }
// }

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
}
