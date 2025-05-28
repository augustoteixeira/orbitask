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

pub async fn create_state(
    db: &mut Connection<Db>,
    board_id: i64,
    name: String,
    is_finished: bool,
) -> Result<State, sqlx::Error> {
    // Step 1: get the current max position for the board
    let max_pos: Option<i64> = sqlx::query_scalar(
        "SELECT MAX(position) FROM states WHERE board_id = ?",
    )
    .bind(board_id)
    .fetch_one(&mut ***db)
    .await?;

    let new_pos = max_pos.unwrap_or(-1) + 1;

    // Step 2: insert the new state
    let state = sqlx::query_as::<_, State>(
        r#"
    INSERT INTO states (board_id, name, is_finished, position)
    VALUES (?, ?, ?, ?)
    RETURNING id, board_id, name, is_finished, position
    "#,
    )
    .bind(board_id)
    .bind(name)
    .bind(is_finished)
    .bind(new_pos)
    .fetch_one(&mut ***db)
    .await?;

    Ok(state)
}

pub async fn delete_state(
    db: &mut Connection<Db>,
    state_id: i64,
) -> Result<Option<State>, DbError> {
    // Step 1: fetch the state if it exists
    let state = sqlx::query_as::<_, State>(
        "SELECT id, board_id, name, is_finished, position FROM states WHERE id = ?"
    )
    .bind(state_id)
    .fetch_optional(&mut ***db)
    .await
    .context(SqlxSnafu)?;

    // 2. If it exists, delete and reorder
    if let Some(ref s) = state {
        // 2a. Delete the state
        sqlx::query("DELETE FROM states WHERE id = ?")
            .bind(state_id)
            .execute(&mut ***db)
            .await
            .context(SqlxSnafu)?;

        // 2b. Shift down all states with higher position
        sqlx::query(
            "UPDATE states
             SET position = position - 1
             WHERE board_id = ? AND position > ?",
        )
        .bind(s.board_id)
        .bind(s.position)
        .execute(&mut ***db)
        .await
        .context(SqlxSnafu)?;
    }

    Ok(state)
}

pub async fn rename_state(
    db: &mut Connection<Db>,
    state_id: i64,
    new_name: String,
) -> Result<Option<State>, DbError> {
    // Step 1: Fetch the state (optional)
    let mut state = sqlx::query_as::<_, State>(
        "SELECT id, board_id, name, is_finished, position FROM states WHERE id = ?"
    )
    .bind(state_id)
    .fetch_optional(&mut ***db)
    .await
    .context(SqlxSnafu)?;

    // Step 2: Update if it exists
    if state.is_some() {
        sqlx::query("UPDATE states SET name = ? WHERE id = ?")
            .bind(&new_name)
            .bind(state_id)
            .execute(&mut ***db)
            .await
            .context(SqlxSnafu)?;

        // Update the name in the struct too
        if let Some(ref mut s) = state {
            s.name = new_name;
        }
    }

    Ok(state)
}
