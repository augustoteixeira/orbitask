use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum DbError {
    #[snafu(display("Database error when {task}: {source}"))]
    SqlxError { task: String, source: sqlx::Error },
    #[snafu(display("Note not found {id}"))]
    NoNoteError { id: i64, source: sqlx::Error },
    #[snafu(display("Lob not found {id}"))]
    NoLogError { id: i64, source: sqlx::Error },
    #[snafu(display("Execution error: {trace}"))]
    ExecutionError { trace: String },
    #[snafu(display("Lua error when {task}: {source}"))]
    LuaError { task: String, source: mlua::Error },
    #[snafu(display("Parsing error when {when}"))]
    ParseError { when: String },
}
