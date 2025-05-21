use maud::{html, Markup};

const CSS: &str =
    "https://cdn.jsdelivr.net/npm/@picocss/pico@2/css/pico.classless.min.css";

fn footer() -> Markup {
    html! {
      footer."container-fluid" {
        small {
          a href="https://github.com/picocss/examples/tree/master/v1-sign-in/"
                class="secondary" {
            "Source code"
          }
        }
      }
    }
}

#[get("/login")]
pub async fn login() -> Markup {
    let markup = html! {
        link rel="stylesheet"
        href=(CSS);

        main.container {
          article.grid {
            div {
              hgroup {
                h1 { "Sign in" }
              }
              form method="post" action="/login" {
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
