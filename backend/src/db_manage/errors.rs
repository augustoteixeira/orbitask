use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum DbError {
    #[snafu(display("Database error"))]
    SqlxError { source: sqlx::Error },
    #[snafu(display("Note not found {id}"))]
    NoNoteError { id: i64 },
    #[snafu(display("Execution error: {trace}"))]
    ExecutionError { trace: String },
}
