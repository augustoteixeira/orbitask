use crate::sqlx::FromRow;
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;

use super::Db;

#[derive(Debug, FromRow)]
pub struct State {
    pub id: i64,
    pub board_id: i64,
    pub name: String,
    pub is_finished: bool,
    pub position: i64,
}

pub async fn get_states_for_board(
    db: &mut Connection<Db>,
    board_id: i64,
) -> Result<Vec<State>, sqlx::Error> {
    let states = sqlx::query_as::<_, State>(
        "SELECT id, board_id, name, is_finished, position
         FROM states
         WHERE board_id = ?
         ORDER BY position",
    )
    .bind(board_id)
    .fetch_all(&mut ***db)
    .await?;

    Ok(states)
}
