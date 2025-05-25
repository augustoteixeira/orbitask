use crate::db_manage::get_states_for_board;
use crate::sqlx::{FromRow, Row};
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;

use super::Db;

#[derive(Debug, FromRow)]
pub struct Board {
    pub id: i64,
    pub name: String,
    pub is_template: bool,
}

pub async fn get_board(
    db: &mut Connection<Db>,
    board_id: i64,
) -> Result<Option<Board>, sqlx::Error> {
    let board = sqlx::query_as::<_, Board>(
        "SELECT id, name, is_template FROM boards WHERE id = ?",
    )
    .bind(board_id)
    .fetch_optional(&mut ***db)
    .await?;

    Ok(board)
}

pub async fn create_board(
    db: &mut Connection<Db>,
    name: String,
    is_template: bool,
    template_id: Option<i64>,
) -> Result<i64, sqlx::Error> {
    // 1. Insert new board
    let row = sqlx::query(
        "INSERT INTO boards (name, is_template) VALUES (?, ?) RETURNING id",
    )
    .bind(&name)
    .bind(is_template)
    .fetch_one(&mut ***db)
    .await?;

    let new_board_id: i64 = row.get("id");

    // 2. If template is provided, copy states from it
    if let Some(template_id) = template_id {
        let states = get_states_for_board(db, template_id).await?;

        for state in states {
            sqlx::query(
                "INSERT INTO states (board_id, name, is_finished, position)
                 VALUES (?, ?, ?, ?)",
            )
            .bind(new_board_id)
            .bind(&state.name)
            .bind(state.is_finished)
            .bind(state.position)
            .execute(&mut ***db)
            .await?;
        }
    }

    Ok(new_board_id)
}

pub async fn get_all_boards(
    db: &mut Connection<Db>,
    include_templates: bool,
) -> Result<Vec<Board>, sqlx::Error> {
    let rows: Vec<Board> = sqlx::query_as::<_, Board>(
        format!(
            r#"
      SELECT id, name, is_template
      FROM boards
      {:}
      ORDER BY name
        "#,
            if include_templates {
                "".to_string()
            } else {
                "WHERE is_template = 0".to_string()
            }
        )
        .as_str(),
    )
    .fetch_all(&mut ***db)
    .await?;

    let boards = rows
        .into_iter()
        .map(|row| Board {
            id: row.id,
            name: row.name,
            is_template: row.is_template,
        })
        .collect();

    Ok(boards)
}
