use chrono::NaiveDate;
use std::collections::HashMap;

use crate::{
    api::codes::{Action, FormType, Value},
    sqlx::{FromRow, Row},
};
use rocket_db_pools::Connection;

use crate::Db;

#[derive(Debug, FromRow)]
pub struct Code {
    pub name: String,
    pub script: String,
}

pub async fn create_code(
    db: &mut Connection<Db>,
    name: String,
    code: String,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
    INSERT INTO codes (name, code)
    VALUES (?, ?)
    "#,
    )
    .bind(name)
    .bind(code)
    .fetch_one(&mut ***db)
    .await?;

    let new_attribute_id: i64 = row.get("id");
    Ok(new_attribute_id)
}

pub async fn get_code(
    db: &mut Connection<Db>,
    note_id: i64,
) -> Result<Option<Code>, sqlx::Error> {
    let code = sqlx::query_as::<_, Code>(
        r#"
        SELECT codes.name, codes.script
        FROM notes
        JOIN codes ON codes.name = notes.code_name
        WHERE notes.id = ?
        "#,
    )
    .bind(note_id)
    .fetch_optional(&mut ***db)
    .await?;

    Ok(code)
}

pub async fn get_all_code_names(
    db: &mut Connection<Db>,
) -> Result<Vec<String>, sqlx::Error> {
    let names =
        sqlx::query_scalar::<_, String>("SELECT name FROM codes ORDER BY name")
            .fetch_all(&mut ***db)
            .await?;

    Ok(names)
}

pub fn parse_fields(
    form_type: &FormType,
    inputs: &HashMap<String, String>,
    prefix: &String,
) -> Result<Value, String> {
    match form_type {
        FormType::UInt => {
            let value = inputs
                .get(prefix)
                .ok_or(format!("Missing field: {:?}", prefix))?;
            value
                .parse::<u64>()
                .map(Value::UInt)
                .map_err(|e| format!("Invalid integer: {e}").into())
        }
        FormType::Date => {
            let value = inputs
                .get(prefix)
                .ok_or(format!("Missing field: {:?}", prefix))?;
            NaiveDate::parse_from_str(value, "%Y-%m-%d")
                .map(Value::Date)
                .map_err(|e| format!("Invalid date: {e}").into())
        }
    }
}

// pub fn get_form_type(id: i64, action_name: String) -> FormType {
//     FormType::UInt(UIntField {
//         label: "my_int".to_string(),
//         title: "This is a placeholder title".to_string(),
//     })
// }

pub async fn get_forms(
    db: &mut Connection<Db>,
    id: i64,
) -> HashMap<String, Action> {
    let code = get_code(db, id).await.unwrap();
    match code {
        Some(_) => {
            let mut result: HashMap<String, Action> = HashMap::new();
            result.insert(
                "crazy".to_string(),
                Action {
                    label: "crazy".to_string(),
                    title: "Crazy code!!!".to_string(),
                    form_type: FormType::UInt,
                },
            );
            result
        }
        None => {
            let mut result: HashMap<String, Action> = HashMap::new();
            result.insert(
                "done".to_string(),
                Action {
                    label: "done".to_string(),
                    title: "Mark as done".to_string(),
                    form_type: FormType::Date,
                },
            );
            result
        }
    }
}
