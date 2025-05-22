use crate::Db;
use maud::{html, Markup};
use rocket::response::Redirect;

use super::api::User;
use crate::api::require_auth;
use crate::db_manage::get_all_boards;
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

#[get("/boards")]
pub async fn boards(
    user: Option<User>,
    db: Connection<Db>,
) -> Result<Markup, Redirect> {
    require_auth(user)?;
    let boards = get_all_boards(db).await.unwrap();
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
                ul {
                  @for board in boards {
                    li {
                      a href={(format!("/boards/{}", board.id))} {
                        (board.name)
                      }
                    }
                  }
                }
              }

              section class="main" {
                h2 { "Welcome to Boards" }
                p { "Select a board to see its notes." }
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
