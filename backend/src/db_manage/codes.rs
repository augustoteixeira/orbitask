use chrono::NaiveDate;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::ResultExt;
use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    api::codes::{Action, Date, FormContainer, FormType, Value},
    db_manage::attributes::{get_attribute, set_attribute},
    sqlx::{FromRow, Row},
};
use mlua::{
    Function, Lua, LuaNativeAsyncFn, LuaSerdeExt, Table, Thread, ThreadStatus,
    Value as LuaValue,
};
use rocket_db_pools::Connection;
use serde_json::Value as JsonValue;

use crate::Db;

use super::{
    errors::{DbError, LuaSnafu, ParseSnafu, SqlxSnafu},
    get_child_notes,
};

#[derive(Debug, FromRow)]
pub struct Code {
    pub name: String,
    pub capabilities: String,
    pub script: String,
}

pub async fn create_code(
    db: &mut Connection<Db>,
    name: String,
    capabilities: String,
    script: String,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
    INSERT INTO codes (name, capabilities, code)
    VALUES (?, ?, ?)
    "#,
    )
    .bind(name)
    .bind(capabilities)
    .bind(script)
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
        SELECT codes.name, codes.capabilities, codes.script
        FROM notes
        JOIN codes ON codes.name = notes.code_name
        WHERE notes.id = ?
        "#,
    )
    .bind(note_id)
    .fetch_optional(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "querying code",
    })?;

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
                .map(|d| Value::Date(Date(d)))
                .map_err(|e| format!("Invalid date: {e}").into())
        }
    }
}

#[derive(Serialize, Deserialize)]
enum Command<T> {
    Result(T),
    SysLog(String),
    SetOwnAttribute { key: String, value: String },
    GetOwnAttribute { key: String },
}

pub fn get_capability_name<T>(command: &Command<T>) -> String {
    match command {
        Command::Result(_) => "Result",
        Command::SysLog(_) => "SysLog",
        Command::SetOwnAttribute { .. } => "SetOwnAttribute",
        Command::GetOwnAttribute { .. } => "GetOwnAttribute",
    }
    .to_string()
}

pub async fn run<R>(
    db: &mut Connection<Db>,
    code: Code,
    command_name: &str,
    id: i64,
    arguments: JsonValue,
) -> Result<R, DbError>
where
    R: DeserializeOwned,
{
    let lua = Lua::new();
    let capabilities: Vec<String> =
        serde_json::from_str(code.capabilities.as_str()).map_err(|e| {
            DbError::ParseError {
                when: format!("loading capabilities: {e}"),
            }
        })?;
    //println!("{}", code.script);
    lua.load(code.script).exec().context(LuaSnafu {
        task: "loading code",
    })?;
    let globals = lua.globals();
    let thread: Thread = globals.get(command_name).context(LuaSnafu {
        task: "getting forms",
    })?;
    let arg_as_value: mlua::Value = lua.to_value(&arguments).unwrap();
    let mut result: mlua::Value =
        thread.resume(arg_as_value).context(LuaSnafu {
            task: "first resuming",
        })?;
    loop {
        let command: Command<R> =
            lua.from_value(result.clone()).map_err(|e| {
                DbError::ParseError {
                    when: format!("deserializing command {:?}: {}", &result, e),
                }
            })?;
        let command_name = get_capability_name(&command);
        if !capabilities.iter().any(|c| c == command_name.as_str()) {
            if command_name != "Result" {
                return Err(DbError::ExecutionError {
                    trace: format!("capability ({command_name}) not provided")
                        .to_string(),
                });
            }
        }
        let response: JsonValue = match command {
            Command::Result(a) => return Ok(a),
            Command::SysLog(s) => {
                println!("{s}");
                ().into()
            }
            Command::SetOwnAttribute { key, value } => {
                set_attribute(db, id, &key.clone(), &value.clone()).await?;
                ().into()
            }
            Command::GetOwnAttribute { key } => {
                get_attribute(db, id, &key.clone()).await?.into()
            }
        };
        match thread.status() {
            ThreadStatus::Finished => {
                return Err(DbError::ExecutionError {
                    trace: "script did not return".to_string(),
                })
            }
            ThreadStatus::Error => {
                return Err(DbError::ExecutionError {
                    trace: "script did not return".to_string(),
                })
            }
            _ => {}
        }
        let response_as_value: mlua::Value = lua.to_value(&response).unwrap();
        result = match response {
            JsonValue::Null => thread.resume(()),
            _ => thread.resume(response_as_value),
        }
        .context(LuaSnafu {
            task: "resuming again",
        })?;
    }
}

pub async fn get_forms(
    db: &mut Connection<Db>,
    id: i64,
) -> Result<HashMap<String, FormContainer>, DbError> {
    let optional_code = get_code(db, id).await?;
    match optional_code {
        Some(code) => {
            let forms = run::<HashMap<String, FormContainer>>(
                db,
                code,
                "forms",
                id,
                JsonValue::Null,
            )
            .await?;
            Ok(forms)
        }
        None => {
            let mut result: HashMap<String, FormContainer> = HashMap::new();
            let children =
                get_child_notes(db, id).await.context(SqlxSnafu {
                    task: "getting children",
                })?;
            if !children.is_empty() {
                return Ok(result);
            }
            let done_status = get_attribute(db, id, "done").await?;
            if let Some(_) = done_status {
                return Ok(result);
            }
            let action = Action {
                label: "done".to_string(),
                title: "Day it was done".to_string(),
                form_type: FormType::Date,
            };
            result.insert(
                "done".to_string(),
                FormContainer {
                    title: "Mark as done".to_string(),
                    label: "done".to_string(),
                    action,
                },
            );
            Ok(result)
        }
    }
}

pub async fn execute(
    db: &mut Connection<Db>,
    id: i64,
    form_container: &FormContainer,
    value: &Value,
) -> Result<String, DbError> {
    let option_code = get_code(db, id).await?;
    match option_code {
        None => execute_done(db, id, &form_container.action, value).await,
        Some(code) => {
            let message = run::<String>(
                db,
                code,
                form_container.label.as_str(),
                id,
                serde_json::from_str(
                    serde_json::to_string(value).unwrap().as_str(),
                )
                .unwrap(),
            )
            .await?;
            Ok(message)
        }
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
    let children = get_child_notes(db, id).await.context(SqlxSnafu {
        task: "getting children",
    })?;
    if !children.is_empty() {
        return Err(DbError::ExecutionError {
            trace: "Cannot mark note as done if it has children".to_string(),
        });
    }
    match value {
        Value::Date(date) => {
            set_attribute(db, id, "done", date.0.to_string().as_str()).await?
        }
        _ => {
            return Err(DbError::ExecutionError {
                trace: format!("{value:?}"),
            });
        }
    }
    Ok("Note marked as done".to_string())
}
