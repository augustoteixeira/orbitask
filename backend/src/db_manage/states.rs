use crate::sqlx::{FromRow, Row};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;

use super::Db;
use crate::db_manage::errors::{DbError, SqlxSnafu};
use snafu::ResultExt;

#[derive(Debug, FromRow)]
pub struct State {
    pub id: i64,
    pub board_id: i64,
    pub name: String,
    pub is_finished: bool,
    pub position: i64,
}

pub async fn get_state(
    db: &mut Connection<Db>,
    state_id: i64,
) -> Result<Option<State>, sqlx::Error> {
    let state = sqlx::query_as::<_, State>(
        r#"SELECT id, board_id, name, is_finished, position
               FROM states WHERE id = ?"#,
    )
    .bind(state_id)
    .fetch_optional(&mut ***db)
    .await?;

    Ok(state)
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

pub async fn move_state(
    db: &mut Connection<Db>,
    state_id: i64,
    old_position: u64,
    new_position: u64,
) -> Result<Option<State>, DbError> {
    // Get the board_id of the state
    let board_id: i64 = sqlx::query("SELECT board_id FROM states WHERE id = ?")
        .bind(state_id)
        .fetch_optional(&mut ***db)
        .await
        .context(SqlxSnafu)?
        .map(|row| row.get("board_id"))
        .ok_or(DbError::NoBoardError { id: state_id })?;

    // No such state
    if board_id == 0 {
        return Err(DbError::NoBoardError { id: state_id });
    }

    let states = get_states_for_board(db, board_id).await.unwrap();
    if old_position as usize >= states.len() {
        return Err(DbError::StateOOBError {
            pos: old_position,
            board_id,
        });
    }
    if new_position as usize >= states.len() {
        return Err(DbError::StateOOBError {
            pos: new_position,
            board_id,
        });
    }

    if old_position == new_position {
        // No move needed
        return sqlx::query_as::<_, State>(
            "SELECT id, board_id, name, is_finished, position FROM states WHERE id = ?"
        )
        .bind(state_id)
        .fetch_optional(&mut ***db)
            .await.context(SqlxSnafu);
    }

    // Shift other states depending on move direction
    if new_position < old_position {
        // Moving up: shift those between new_position and old_position down
        sqlx::query(
            "UPDATE states
             SET position = position + 1
             WHERE board_id = ? AND position >= ? AND position < ?",
        )
        .bind(board_id)
        .bind(new_position as i64)
        .bind(old_position as i64)
        .execute(&mut ***db)
        .await
        .context(SqlxSnafu)?;
    } else {
        // Moving down: shift those between old_position and new_position up
        sqlx::query(
            "UPDATE states
             SET position = position - 1
             WHERE board_id = ? AND position > ? AND position <= ?",
        )
        .bind(board_id)
        .bind(old_position as i64)
        .bind(new_position as i64)
        .execute(&mut ***db)
        .await
        .context(SqlxSnafu)?;
    }

    // Set new position for moved state
    sqlx::query(
        "UPDATE states
         SET position = ?
         WHERE id = ?",
    )
    .bind(new_position as i64)
    .bind(state_id)
    .execute(&mut ***db)
    .await
    .context(SqlxSnafu)?;

    // Return the updated state
    let state = sqlx::query_as::<_, State>(
        "SELECT id, board_id, name, is_finished, position FROM states WHERE id = ?"
    )
    .bind(state_id)
    .fetch_optional(&mut ***db)
    .await.context(SqlxSnafu)?;

    Ok(state)
}
