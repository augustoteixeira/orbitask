use rocket::form::Form;
use rocket::post;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;
use sqlx::FromRow;
use std::collections::HashMap;

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

#[derive(Debug, FromRow)]
pub struct UIntField {
    pub label: String,
    pub title: String,
}

#[derive(Debug)]
pub enum FormType {
    UInt(UIntField),
}

#[derive(Debug)]
pub struct Action {
    pub label: String,
    pub title: String,
    pub form_type: FormType,
}

#[derive(Debug)]
pub enum Value {
    UInt(u64),
}

pub fn parse_fields(
    form_type: &FormType,
    inputs: &HashMap<String, String>,
    prefix: String,
) -> Result<Value, String> {
    match form_type {
        FormType::UInt(field) => {
            let mut key = prefix.clone();
            key.push_str(&field.label.clone());
            let value = inputs
                .get(key.as_str())
                .ok_or(format!("Missing field: {}", key))?;
            value
                .parse::<u64>()
                .map(Value::UInt)
                .map_err(|e| format!("Invalid integer: {e}").into())
        }
    }
}

pub fn get_form_type(id: i64, action_name: String) -> FormType {
    FormType::UInt(UIntField {
        label: "my_int".to_string(),
        title: "This is a placeholder title".to_string(),
    })
}

pub fn get_forms(id: i64) -> Vec<FormType> {
    let mut result = Vec::new();
    result.push(get_form_type(id, "placeholder".to_string()));
    result
}
