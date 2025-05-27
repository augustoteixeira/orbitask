use crate::Db;
use maud::{html, Markup};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

use super::state::{get_state_view, states_grid, StateView};
use super::style::{base_flash, footer, header, meta, sidebar_style};
use super::tags::{render_tags, Tag};
use crate::api::{require_auth, Authenticated};
use crate::db_manage::{
    get_all_boards, get_board, get_states_for_board, get_tags_from_board, Board,
};
use crate::frontend::User;
use crate::sqlx::FromRow;
use rocket_db_pools::Connection;

#[derive(Debug, FromRow)]
pub struct BoardView {
    pub board: Board,
    pub tags: Vec<Tag>,
    pub state_views: Vec<StateView>,
}

pub async fn get_board_view(
    mut db: &mut Connection<Db>,
    id: i64,
) -> Result<BoardView, sqlx::Error> {
    let board = get_board(&mut db, id).await.unwrap().unwrap();
    let tags = get_tags_from_board(&mut db, board.id).await.unwrap();
    let states = get_states_for_board(&mut db, board.id).await.unwrap();
    let mut state_views = Vec::new();
    for s in states {
        state_views.push(get_state_view(&mut db, s.id).await?);
    }
    Ok(BoardView {
        board,
        tags,
        state_views,
    })
}

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
    flash: Option<FlashMessage<'_>>,
    _auth: Authenticated,
    mut db: Connection<Db>,
) -> Result<Markup, Flash<Redirect>> {
    let boards = get_all_boards(&mut db, false).await.unwrap();
    let markup = html! {
      html {
        head {
          (meta())
          title { "Boards" }

          (sidebar_style())
        }
        (base_flash(flash))
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
    flash: Option<FlashMessage<'_>>,
    mut db: Connection<Db>,
) -> Result<Markup, Flash<Redirect>> {
    require_auth(user)?;
    let board_view = get_board_view(&mut db, id).await.unwrap();
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
            (base_flash(flash))
            nav style="margin-bottom: 1rem;" {
              a href="/boards" role="button" {
                "← Back to Boards"
              }
            }

            h1 { (board_view.board.name) }

            @if board_view.board.is_template {
              p style="color: var(--muted-color); font-style: italic;" {
                "This board is a template."
              }
            }

            (render_tags(board_view.tags))

            hr;

            (states_grid(board_view.state_views))
          }
        }
          (footer())
        }
      }
    };
    Ok(markup)
}

#[get("/boards/new")]
pub async fn new_board(
    user: Option<User>,
    flash: Option<FlashMessage<'_>>,
    mut db: Connection<Db>,
) -> Result<Markup, Flash<Redirect>> {
    require_auth(user)?;

    let boards = get_all_boards(&mut db, true).await.unwrap();
    let templates: Vec<(i64, String)> = boards
        .into_iter()
        .filter(|board| board.is_template)
        .map(|board| (board.id, board.name))
        .collect();

    Ok(html! {
        head {
          (meta())
          title { "Boards" }

          (sidebar_style())
        }

      main class="container" {
        (base_flash(flash))
        h1 { "Create a New Board" }

        form method="post" action="/boards/create" {
          label for="name" { "Board name" }
          input type="text" id="name" name="name" required;

          fieldset {
            legend { "How to create?" }

            label {
              input type="radio" name="mode" value="empty" checked;
              " Empty board"
            }

            label {
              input type="radio" name="mode" value="template";
              " Use template"
            }
          }

          // Template picker (initially shown — JS can later hide/show this)
          label for="template_id" {
            "Choose template:"
            select id="template_id" name="template_id" {
              option value="" disabled selected { "-- Select template --" }
              @for (id, name) in &templates {
                option value={(id)} { (name) }
              }
            }
          }

          button type="submit" class="contrast" { "Create Board" }
        }
      }
    })
}
