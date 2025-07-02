use chrono::NaiveDate;
use snafu::ResultExt;
use std::collections::HashMap;

use crate::{
    api::codes::{Action, FormType, Value},
    db_manage::attributes::{get_attribute, set_attribute},
    sqlx::{FromRow, Row},
};
use mlua::{
    FromLua, Function, Lua, MetaMethod, Result as LuaResult, UserData,
    UserDataMethods, Value as LuaValue, Variadic,
};
use rocket_db_pools::Connection;
use serde_json;

use crate::Db;

use super::{
    errors::{DbError, LuaSnafu, SqlxSnafu},
    get_child_notes,
};

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
) -> Result<Option<Code>, DbError> {
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
    .await
    .context(SqlxSnafu)?;

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
) -> Result<HashMap<String, Action>, DbError> {
    let code = get_code(db, id).await?;
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
            let lua = Lua::new();
            let lua_forms_function = r#"
              return '{ "crazy": { "label": "crazy", "title": "Crazy code!!!", "form_type": "UInt" } }'
            "#;
            let lua_forms_string = lua
                .load(lua_forms_function)
                .eval::<String>()
                .context(LuaSnafu)?;
            Ok(serde_json::from_str(lua_forms_string.as_str()).unwrap()) // TODO remove this unrwrap
        }
        None => {
            let mut result: HashMap<String, Action> = HashMap::new();
            let children = get_child_notes(db, id).await.context(SqlxSnafu)?;
            if !children.is_empty() {
                return Ok(result);
            }
            let done_status = get_attribute(db, id, "done").await?;
            if let Some(_) = done_status {
                return Ok(result);
            }
            result.insert(
                "done".to_string(),
                Action {
                    label: "done".to_string(),
                    title: "Mark as done".to_string(),
                    form_type: FormType::Date,
                },
            );
            println!("{:?}", serde_json::to_string(&result));
            Ok(result)
        }
    }
}

pub async fn execute(
    db: &mut Connection<Db>,
    id: i64,
    action: &Action,
    value: &Value,
) -> Result<String, DbError> {
    let option_code = get_code(db, id).await?;
    match option_code {
        None => execute_done(db, id, action, value).await,
        Some(_code) => unimplemented!(),
    }
}

pub async fn execute_done(
    db: &mut Connection<Db>,
    id: i64,
    action: &Action,
    value: &Value,
) -> Result<String, DbError> {
    if action.label != "done" {
        return Err(DbError::ExecutionError {
            trace: format!(
                "A note without code can only handle <done>, not <{}>",
                action.label
            )
            .to_string(),
        });
    }
    let children = get_child_notes(db, id).await.context(SqlxSnafu)?;
    if !children.is_empty() {
        return Err(DbError::ExecutionError {
            trace: "Cannot mark note as done if it has children".to_string(),
        });
    }
    match value {
        Value::Date(date) => {
            set_attribute(db, id, "done", date.to_string().as_str()).await?
        }
        _ => {
            return Err(DbError::ExecutionError {
                trace: format!("{value:?}"),
            });
        }
    }
    Ok("Note marked as done".to_string())
}
