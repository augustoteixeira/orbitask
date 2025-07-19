use maud::{html, Markup};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Responder, Result};
use rocket::uri;
use rocket::{Request, Response};

use crate::db_manage::Note;
use crate::frontend::codes::rocket_uri_macro_list_codes;
use crate::frontend::notes::rocket_uri_macro_root_notes;
use crate::frontend::render::render_notes_grid;
use crate::frontend::style::{base_flash, footer, meta};

#[derive(Debug, Clone)]
pub enum MyFlashType {
    Success,
    Warning,
    Error,
    Info,
}

#[derive(Debug, Clone)]
pub struct MyFlash {
    pub flash_type: MyFlashType,
    pub message: Markup,
}

impl<'a> From<FlashMessage<'a>> for MyFlash {
    fn from(flash: FlashMessage<'a>) -> Self {
        let flash_type = match flash.kind() {
            "success" => MyFlashType::Success,
            "warning" => MyFlashType::Warning,
            "error" => MyFlashType::Error,
            _ => MyFlashType::Info,
        };
        MyFlash {
            flash_type,
            message: html! { (flash.message()) },
        }
    }
}

#[derive(Debug)]
pub enum ViewState {
    Login,
    Root(Vec<Note>),
}

#[derive(Debug)]
pub struct View {
    pub state: ViewState,
    pub flash: Vec<MyFlash>,
}

fn render_flashes(flashes: Vec<MyFlash>) -> Markup {
    html! {
      @for flash in flashes {
          @match flash.flash_type {
              MyFlashType::Success => { p class="flash-success" { (flash.message) } },
              MyFlashType::Warning => { p class="flash-warning" { (flash.message) } },
              MyFlashType::Info => { p class="flash-info" { (flash.message) } },
              MyFlashType::Error => { p class="flash-error" { (flash.message) } },
          }
      }
    }
}

fn login() -> Markup {
    html! {
      (meta())
      link rel="stylesheet" href="/static/style.css";

      main.container {
        article.grid {
          div {
            hgroup {
              h1 { "Sign in" }
            }
            form method="post" action="/login?next=/" {
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
      }
      (footer())
    }
}

fn root(notes: Vec<Note>) -> Markup {
    html! {
      main {
        section class="main" {
          h2 { "Home" }
          (render_notes_grid(&notes))
          a href="/notes/new" role="button" { "Create New Root Note" }
        }
      }
    }
}

pub struct Page {
    pub title: Markup,
    pub main: Markup,
    pub flash: Markup,
}

pub fn header() -> maud::Markup {
    html! {
    header {
        div class="header-left" {
          h1 { "Orbitask" }
        }
        div class="header-right" {
          a href={(uri!(list_codes()))} role="button" {
            "Codes"
          }
          a href={(uri!(root_notes()))} role="button" {
            "Notes"
          }
          form method="post" action="/logout" {
            button type="submit" { "Logout" }
          }
        }
      }
    }
}

pub fn render(page: Page) -> Markup {
    html! {
      (maud::DOCTYPE)
      html lang="en" {
        head {
          meta charset="utf-8";
          meta name="viewport" content="width=device-width, initial-scale=1";

          (page.title)

          link rel="stylesheet" href="/static/style.css";
          link rel="icon" type="image/x-icon" href="/static/favicon.png";
        }

        body {
          main {
            (header())
            (page.flash)
            (page.main)
            //(footer())
          }
        }
      }
    }
}

impl<'r> Responder<'r, 'static> for View {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        let main = match self.state {
            ViewState::Login => login(),
            ViewState::Root(notes) => root(notes),
        };
        let rendered_flash = render_flashes(self.flash);
        let markup = html! {
            (rendered_flash)
            (main)
        };
        //markup.respond_to(req)
        let page = Page {
            title: html! {title {"Notes"}},
            flash: rendered_flash,
            main,
        };
        render(page).respond_to(req)
    }
}
