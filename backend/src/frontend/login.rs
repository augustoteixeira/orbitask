use maud::{html, Markup};

use super::style::{base_flash, footer, meta};
use rocket::request::FlashMessage;

#[get("/login")]
pub async fn login(flash: Option<FlashMessage<'_>>) -> Markup {
    let markup = html! {
        (meta())
        main.container {
          (base_flash(flash))
          article.grid {
            div {
              hgroup {
                h1 { "Sign in" }
              }
              form method="post" action="/login?next=/boards" {
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
