use maud::{html, Markup};

use super::style::{footer, meta};

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
