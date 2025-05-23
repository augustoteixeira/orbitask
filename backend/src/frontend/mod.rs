use crate::Db;
use maud::{html, Markup};
use rocket::response::Redirect;

use super::api::User;
use crate::api::require_auth;
use crate::db_manage::{get_all_boards, get_board, Board};
use rocket_db_pools::Connection;

pub fn header() -> Markup {
    html! {
      header {
        nav {
          h1 { "Orbitask" }
        }
      }
    }
}

fn footer() -> Markup {
    html! {
      footer."container-fluid" {
      small {
        a href="https://github.com/augustoteixeira/orbitasks"
          class="secondary" {
        "Source code"
        }
      }
      }
    }
}

fn meta() -> Markup {
    html! {
      meta charset="utf-8";
      meta name="viewport" content="width=device-width, initial-scale=1.0";
      link rel="stylesheet"
           href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css";
    }
}

#[get("/login")]
pub async fn login() -> Markup {
    let markup = html! {
        (meta())

        main.container {
          article.grid {
            div {
              hgroup {
                h1 { "Sign in" }
              }
              form method="post" action="/login?next=/dashboard" {
                input type="password" name="password" placeholder="Password"
                  aria-label="Password" autocomplete="current-password"
                  required;

                fieldset {
                  label for="remember" {
                    input type="checkbox" role="switch"
                      id="remember" name="remember";
                    "Remember me (not implemented yet)"
                  }
                }

                button type="submit" class="contrast" { "Login" }
            }
          }
        }
        (footer())
      }

    };
    markup
}

fn sidebar_style() -> Markup {
    html! {
      style {
        r#"
    .layout {
      display: flex;
      flex-direction: column;
      gap: 2rem;
    }

    @media (min-width: 768px) {
      .layout {
        flex-direction: row;
      }

      .sidebar {
        flex: 1;
        max-width: 250px;
      }

      .main {
        flex: 3;
      }
    }

    .sidebar {
      background-color: var(--muted-bg);
      padding: 1rem;
      border-radius: 0.5rem;
    }
    "#
      }
    }
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
    user: Option<User>,
    db: Connection<Db>,
) -> Result<Markup, Redirect> {
    require_auth(user)?;
    let boards = get_all_boards(db, false).await.unwrap();
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
    db: Connection<Db>,
) -> Result<Markup, Redirect> {
    require_auth(user)?;
    let board = get_board(db, id).await.unwrap().unwrap();
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

            hr;

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
