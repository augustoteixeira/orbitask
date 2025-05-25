use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub(crate)))]
pub enum DbError {
    #[snafu(display("Database error"))]
    SqlxError { source: sqlx::Error },
    #[snafu(display("Board not found {id}"))]
    NoBoardError { id: i64 },
    #[snafu(display("Out of bounds state {pos} for board {board_id}"))]
    StateOOBError { pos: u64, board_id: i64 },
}
