use crate::Db;
use maud::{html, Markup};
use rocket::response::Redirect;

use super::frontend_state::states_grid;
use super::frontend_style::{footer, header, meta, sidebar_style};
use super::frontend_tags::render_tags;
use crate::api::require_auth;
use crate::db_manage::{
    get_all_boards, get_board, get_states_for_board, get_tags_from_board, Board,
};
use crate::frontend::User;
use rocket_db_pools::Connection;

pub fn boards_grid(boards: Vec<Board>) -> Markup {
    html! {
      section style=r#"
        display: grid; gap: 1rem;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      "# {
        @for board in boards {
          article style=r#"
            padding: 1rem; border: 1px solid var(--muted-border);
            border-radius: 0.5rem;
          "# {
              a href={(format!("/boards/{}", board.id))} {
                (board.name)
              }
            @if board.is_template {
              p style="font-size: 0.8em; color: var(--muted-color);" {
                "Template"
              }
            }
          }
        }
      }
    }
}

#[get("/boards")]
pub async fn boards(
    user: Option<User>,
    mut db: Connection<Db>,
) -> Result<Markup, Redirect> {
    require_auth(user)?;
    let boards = get_all_boards(&mut db, false).await.unwrap();
    let markup = html! {
      html {
        head {
          (meta())
          title { "Boards" }

          (sidebar_style())
        }
        body {
          (header())

          main {
            section class="layout" {
              aside class="sidebar" {
                h2 { "Boards" }
              }

              section class="main" {
                h2 { "Welcome to your boards" }
                p { "Select a board to see its notes." }
                (boards_grid(boards))
                a href="/boards/new" role="button" { "Create New Board" }
              }
            }
          }

          (footer())
        }
      }
    };
    Ok(markup)
}

#[get("/boards/<id>")]
pub async fn board(
    id: i64,
    user: Option<User>,
    mut db: Connection<Db>,
) -> Result<Markup, Redirect> {
    require_auth(user)?;
    let board = get_board(&mut db, id).await.unwrap().unwrap();
    let tags = get_tags_from_board(&mut db, board.id).await.unwrap();
    let states = get_states_for_board(&mut db, board.id).await.unwrap();
    let markup = html! {
      html {
        head {
          (meta())
          title { "Board" }

          (sidebar_style())
        }
        body {
          (header())

        main class="container" {
          section {
            nav style="margin-bottom: 1rem;" {
              a href="/boards" role="button" {
                "← Back to Boards"
              }
            }

            h1 { (board.name) }

            @if board.is_template {
              p style="color: var(--muted-color); font-style: italic;" {
                "This board is a template."
              }
            }

            (render_tags(tags))

            hr;

            (states_grid(states))
            section style="margin-top: 2rem;" {
              h2 { "Notes" }
              p { "No notes yet." }

              // You could later render states & notes here
            }
          }
        }
          (footer())
        }
      }
    };
    Ok(markup)
}
