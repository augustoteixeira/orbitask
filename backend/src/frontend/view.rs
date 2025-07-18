use maud::{html, Markup};
use rocket::request::FlashMessage;
use rocket::response::{Flash, Responder, Result};
use rocket::{Request, Response};

use crate::frontend::style::{base_flash, footer, meta};

#[derive(Debug)]
pub enum MyFlashType {
    Success,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct MyFlash {
    flash_type: MyFlashType,
    message: Markup,
}

#[derive(Debug)]
pub enum ViewState {
    Login,
}

#[derive(Debug)]
pub struct View {
    pub state: ViewState,
    pub flash: Vec<MyFlash>,
}

fn login() -> Markup {
    html! {
                (meta())
                main.container {
                  //(base_flash(self.flash))
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
                (footer())
              }

    }
}

impl<'r> Responder<'r, 'static> for View {
    fn respond_to(self, req: &'r Request<'_>) -> Result<'static> {
        let markup = match self.state {
            ViewState::Login => login(),
        };
        markup.respond_to(req)
    }
}
