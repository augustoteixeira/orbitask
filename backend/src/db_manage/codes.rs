use chrono::NaiveDate;
use rocket_db_pools::sqlx::FromRow;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use snafu::ResultExt;
use sqlx::SqliteConnection;
use std::collections::HashMap;
use std::fmt::Debug;

use crate::{
    api::codes::{Action, Date, FormContainer, FormType, Value},
    db_manage::attributes::{get_attribute, set_attribute},
    db_manage::notes::create_note,
};
use mlua::{Lua, LuaSerdeExt, Thread, ThreadStatus};
use rocket_db_pools::Connection;
use serde_json::Value as JsonValue;

use crate::db_manage::Db;

use super::{
    errors::{DbError, LuaSnafu, SqlxSnafu},
    get_child_notes,
    logs::create_log,
};

#[allow(dead_code)]
#[derive(Debug, FromRow, Serialize, Deserialize)]
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
) -> Result<String, DbError> {
    sqlx::query(
        r#"
    INSERT INTO codes (name, capabilities, script)
    VALUES (?, ?, ?)
        "#,
    )
    .bind(name)
    .bind(capabilities)
    .bind(script)
    .execute(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "creating code",
    })?;
    let new_code_id: (String,) = sqlx::query_as("SELECT last_insert_rowid()")
        .fetch_one(&mut ***db)
        .await
        .context(SqlxSnafu {
            task: "getting created code",
        })?;
    Ok(new_code_id.0)
}

pub async fn get_code(
    db: &mut SqliteConnection,
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
    .fetch_optional(&mut *db)
    .await
    .context(SqlxSnafu {
        task: "querying code",
    })?;
    Ok(code)
}

pub async fn edit_code(
    db: &mut Connection<Db>,
    name: &str,
    new_capabilities: &str,
    new_script: &str,
) -> Result<(), DbError> {
    sqlx::query(
        r#"
        UPDATE codes
        SET capabilities = ?, script = ?
        WHERE name = ?
        "#,
    )
    .bind(new_capabilities)
    .bind(new_script)
    .bind(name)
    .execute(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "editing code",
    })?;
    Ok(())
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
        FormType::Empty => Ok(Value::Empty),
    }
}

#[derive(Debug, Serialize, Deserialize)]
enum Command<T: Debug> {
    GetId,
    Result(T),
    SysLog(String),
    SetAttribute {
        id: i64,
        key: String,
        value: String,
    },
    GetAttribute {
        id: i64,
        key: String,
    },
    CreateChild {
        parent_id: Option<i64>,
        title: String,
        description: String,
        code_name: Option<String>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Range {
    Own,
    // Decendants,
    // Ancestors,
    // All
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Capabilities {
    SysLog,
    GetAttribute(Range),
    SetAttribute(Range),
    CreateChild(Range),
}

fn within_range(
    _db: &mut SqliteConnection,
    range: &Range,
    id: i64,
    _target_id: Option<i64>,
) -> bool {
    match range {
        Range::Own => matches!(Some(id), _target_id),
    }
}

fn authorized<R: Debug>(
    db: &mut SqliteConnection,
    id: i64,
    command: &Command<R>,
    capability: &Capabilities,
) -> bool {
    match command {
        Command::GetId => true,
        Command::Result(_) => true,
        Command::SysLog(_) => matches!(capability, Capabilities::SysLog),
        Command::GetAttribute { id: target_id, .. } => {
            if let Capabilities::GetAttribute(r) = capability {
                return within_range(db, r, id, Some(*target_id));
            } else {
                false
            }
        }
        Command::SetAttribute { id: target_id, .. } => {
            if let Capabilities::SetAttribute(r) = capability {
                return within_range(db, r, id, Some(*target_id));
            } else {
                false
            }
        }
        Command::CreateChild { parent_id, .. } => {
            if let Capabilities::CreateChild(r) = capability {
                return within_range(db, r, id, *parent_id);
            } else {
                false
            }
        }
    }
}

pub async fn run<R: Debug>(
    db: &mut SqliteConnection,
    code: Code,
    command_name: &str,
    id: i64,
    arguments: JsonValue,
) -> Result<R, DbError>
where
    R: DeserializeOwned,
{
    let lua = Lua::new();
    let capabilities: Vec<Capabilities> =
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
                    when: format!(
                        "deserializing command {:?}: {}",
                        serde_json::to_string(&result),
                        e
                    ),
                }
            })?;
        if !capabilities
            .iter()
            .any(|c| authorized::<R>(db, id, &command, c))
        {
            return Err(DbError::ExecutionError {
                trace: format!(
                    "command {command:?} not authorized in {capabilities:?}"
                )
                .to_string(),
            });
        }
        let response: JsonValue = match command {
            Command::GetId => id.into(),
            Command::Result(a) => return Ok(a),
            Command::SysLog(s) => {
                println!("{s}");
                ().into()
            }
            Command::SetAttribute { id, key, value } => {
                set_attribute(db, id, &key.clone(), &value.clone()).await?;
                ().into()
            }
            Command::GetAttribute { id, key } => {
                get_attribute(db, id, &key.clone()).await?.into()
            }
            Command::CreateChild {
                parent_id,
                title,
                description,
                code_name,
            } => create_note(db, parent_id, title, description, code_name)
                .await?
                .into(),
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
    db: &mut SqliteConnection,
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
            let _ = create_log(
                &mut *db,
                id,
                "info".to_string(),
                format!("Note {id} executed form {form_container:?} with value {value:?}"),
                None,
            ).await?;
            Ok(message)
        }
    }
}

pub async fn execute_done(
    db: &mut SqliteConnection,
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

pub async fn get_code_by_name(
    db: &mut Connection<Db>,
    name: &str,
) -> Result<Option<Code>, DbError> {
    let code = sqlx::query_as::<_, Code>(
        r#"
        SELECT name, capabilities, script
        FROM codes
        WHERE name = ?
        "#,
    )
    .bind(name)
    .fetch_optional(&mut ***db)
    .await
    .context(SqlxSnafu {
        task: "loading code by name",
    })?;
    Ok(code)
}
