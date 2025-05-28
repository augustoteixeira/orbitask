use crate::Db;
use maud::{html, Markup};
use rocket::http::uri;
use rocket::request::FlashMessage;
use rocket::response::{Flash, Redirect};

use super::state::{get_state_view, states_grid, StateView};
use super::style::{base_flash, render, Page};
use super::tags::{render_tags, Tag};
use crate::api::boards::rocket_uri_macro_create_note_submit;
use crate::api::Authenticated;
use crate::db_manage::{
    get_all_boards, get_board, get_states_for_board, get_tags_from_board, Board,
};
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
            border-radius: 0.5rem; margin: 0.5rem;
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
    let contents = html! {
          main {
            section class="main" {
              h2 { "Welcome to your boards" }
              (boards_grid(boards))
              a href="/boards/new" role="button" { "Create New Board" }
            }
          }
    };
    let page = Page {
        title: html! {title {"Boards"}},
        flash: base_flash(flash),
        contents,
    };
    return Ok(render(page));
}

#[get("/boards/<id>")]
pub async fn board(
    id: i64,
    _auth: Authenticated,
    flash: Option<FlashMessage<'_>>,
    mut db: Connection<Db>,
) -> Result<Markup, Flash<Redirect>> {
    let board_view = get_board_view(&mut db, id).await.unwrap();
    let contents = html! {
      main class="container" {
        section {
          nav style="margin-bottom: 1rem;" {
            a href="/boards" role="button" {
              "← Back to Boards"
            }
          }
          // Title + Settings aligned
          div style=r#"display: flex; justify-content: space-between;
                       align-items: center;"# {
            h1 style="margin: 0;" { (board_view.board.name) }
            a href={(format!("/boards/{}/settings", board_view.board.id))}
                             title="Board settings" {
              "Settings"
            }
          }

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
    };
    let page = Page {
        title: html! {title {"Boards"}},
        flash: base_flash(flash),
        contents,
    };
    return Ok(render(page));
}

#[get("/boards/<id>/settings")]
pub async fn board_settings(
    id: i64,
    _auth: Authenticated,
    flash: Option<FlashMessage<'_>>,
    mut db: Connection<Db>,
) -> Result<Markup, Flash<Redirect>> {
    let board_view = get_board_view(&mut db, id).await.unwrap();
    let contents = html! {
        main class="container" {
          section {
            nav style="margin-bottom: 1rem;" {
              a href={(format!("/boards/{}", id))} role="button" {
                "← Back to Board"
              }
            }
            h1 { "Board Settings" }
            h2 { "Add new state" }
            form method="post"
              action={(format!("/boards/{}/create_state",
                               board_view.board.id))}
              style=r#"display: flex; gap: 0.5rem;
                       align-items: center; margin-bottom: 1rem;"#
            {
              input
              type="text"
              name="name"
              placeholder="New state name"
              required
              style=r#"flex-grow: 1; min-width: 150px; padding: 0.25rem;
                       font-size: 0.9em;"#;
              label style="font-size: 0.9em;" {
                input type="checkbox" name="is_finished";
                " Finished?"
              }
              button
                type="submit"
                class="secondary"
              {
                "Create"
              }
            }
            h2 { "States" }
            ul style="list-style: none; padding: 0;" {
              @for state_view in &board_view.state_views {
                @let state = &state_view.state;
                  article style=r#"margin-bottom: 1rem; padding: 1rem;
                                   border: 1px solid var(--muted-border);
                                   border-radius: 0.5rem;"#
                  {
                    h3 style="margin-bottom: 0.5rem;" { (state.name) }
                    form method="post"
                         action={(format!("/states/{}/rename", state.id))}
                         style=r#"display: flex; gap: 0.5rem;
                                  align-items: center; width: 100%;"#
                    {
                      input
                      type="text"
                      name="new_name"
                      value={(state.name)}
                      required
                      style=r#"flex-grow: 1; min-width: 100px; padding:
                               0.25rem; font-size: 0.9em;"#;
                    button
                      type="submit"
                      class="secondary"
                    {
                      "Rename"
                    }
                  }
                  form method="post"
                    action={(format!("/states/{}/delete", state.id))}
                    style="margin-top: 0.5rem;" {
                      button
                      type="submit"
                      class="contrast"
                  {
                    "Delete"
                  }
                }
              }
            }
          }
        }
      }
    };
    let page = Page {
        title: html! {title {"Boards"}},
        flash: base_flash(flash),
        contents,
    };
    return Ok(render(page));
}

#[get("/boards/new")]
pub async fn new_board(
    _auth: Authenticated,
    flash: Option<FlashMessage<'_>>,
    mut db: Connection<Db>,
) -> Result<Markup, Flash<Redirect>> {
    let boards = get_all_boards(&mut db, true).await.unwrap();
    let templates: Vec<(i64, String)> = boards
        .into_iter()
        .filter(|board| board.is_template)
        .map(|board| (board.id, board.name))
        .collect();
    let contents = html! {
      main class="container" {
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
    };

    let page = Page {
        title: html! {title {"Boards"}},
        flash: base_flash(flash),
        contents,
    };
    return Ok(render(page));
}

#[get("/boards/<board_id>/<state_id>/new_note")]
pub async fn new_note(
    _auth: Authenticated,
    board_id: i64,
    state_id: i64,
    flash: Option<FlashMessage<'_>>,
) -> Result<Markup, Flash<Redirect>> {
    let contents = html! {
      main class="container" {
        h1 { "Create a New Note" }

        form method="post" action={
            //(format!("/boards/{}/{}", board_id, state_id) + "/create_note")
            (uri!(create_note_submit(board_id, state_id)))
        } {
          label for="name" { "Note name" }
          input type="text" id="name" name="name" required;

          label for="start_date" { "Start date (YYYY-MM-DD)" }
          input type="text" id="start_date" name="start_date"
                placeholder="2025-06-01" required;

          label for="due_date" { "Due date (YYYY-MM-DD)" }
          input type="text" id="due_date" name="due_date"
                placeholder="2025-06-10" required;

          button type="submit" class="contrast" { "Create Note" }
        }
      }
    };

    let page = Page {
        title: html! { title { "New Note" } },
        flash: base_flash(flash),
        contents,
    };
    Ok(render(page))
}
