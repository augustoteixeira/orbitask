use crate::sqlx::FromRow;
use rocket_db_pools::sqlx::{self};
use rocket_db_pools::Connection;

use super::Db;

#[derive(Debug, FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

pub async fn get_tag(
    mut db: Connection<Db>,
    tag_id: i64,
) -> Result<Option<Tag>, sqlx::Error> {
    let board =
        sqlx::query_as::<_, Tag>("SELECT id, name FROM tags WHERE id = ?")
            .bind(tag_id)
            .fetch_optional(&mut **db)
            .await?;

    Ok(board)
}

pub async fn get_all_tags(
    mut db: Connection<Db>,
) -> Result<Vec<Tag>, sqlx::Error> {
    let tags =
        sqlx::query_as::<_, Tag>("SELECT id, name FROM tags ORDER BY name")
            .fetch_all(&mut **db)
            .await?;

    Ok(tags)
}

pub async fn get_tags_from_board(
    db: &mut Connection<Db>,
    board_id: i64,
) -> Result<Vec<Tag>, sqlx::Error> {
    let tags = sqlx::query_as::<_, Tag>(
        r#"
        SELECT t.id, t.name
        FROM tags t
        INNER JOIN board_tags bt ON t.id = bt.tag_id
        WHERE bt.board_id = ?
        ORDER BY t.name
        "#,
    )
    .bind(board_id)
    .fetch_all(&mut ***db)
    .await?;

    Ok(tags)
}
