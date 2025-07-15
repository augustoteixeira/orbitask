use crate::frontend::codes::rocket_uri_macro_list_codes;
use crate::frontend::notes::rocket_uri_macro_root_notes;
use maud::{html, Markup};
use rocket::request::FlashMessage;
use rocket::uri;

pub struct Page {
    pub title: Markup,
    pub contents: Markup,
    pub flash: Markup,
}

pub fn render(page: Page) -> Markup {
    html! {
      (maud::DOCTYPE)
      html lang="en" {
        head {
          meta charset="utf-8";
          meta name="viewport" content="width=device-width, initial-scale=1";

          (page.title)
          (page.flash)

          // Pico.css CDN
          link rel="stylesheet"
               href="https://unpkg.com/@picocss/pico@1.5.10/css/pico.min.css";
          link rel="icon" type="image/x-icon" href="/static/favicon.png";
          // Optional custom styles
          //link rel="stylesheet" href="/static/style.css";
        }

        body {
          main class="container" {
            (header())

            (page.contents)

            (footer())
          }
        }
      }
    }
}

pub fn base_flash(flash: Option<FlashMessage<'_>>) -> Markup {
    html! {
      @if let Some(msg) = flash {
        footer."container-fluid" {
          p style={
            @let base = r#"margin: 1rem 0; padding: 0.75rem;
                           border-radius: 0.5rem; font-weight: bold;"#;
            @match msg.kind() {
              "success" => (format!("{base} background-color: #d1e7dd; color: #0f5132;")),
              "error" => (format!("{base} background-color: #f8d7da; color: #842029;")),
              _ => (format!(r#"{base} background-color: var(--muted-bg);
                      color: var(--contrast);"#)),
            }
          } {
            (msg.message())
          }
        }
      }
    }
}

pub fn header() -> maud::Markup {
    html! {
      header {
        nav style=r#"display: flex; justify-content: space-between;
                     align-items: center;"# {
          h1 style=r#"margin-bottom: 0"# { "Orbitask" }

          a href={(uri!(list_codes()))} role="button" {
            "Codes"
          }
          a href={(uri!(root_notes()))} role="button" {
            "Notes"
          }

          form method="post" action="/logout" style="margin-bottom: 0" {
            button type="submit" class="secondary"
                   style="padding: 0.2em 0.75em;" { "Logout" }
          }
        }
      }
    }
}

pub fn footer() -> Markup {
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

pub fn meta() -> Markup {
    html! {
      meta charset="utf-8";
      meta name="viewport" content="width=device-width, initial-scale=1.0";
      link rel="stylesheet"
      href=r#"https://unpkg.com/@picocss/pico@latest/css/
              pico.classless.min.css"#;
    }
}

// pub fn sidebar_style() -> Markup {
//     html! {
//       style {
//         r#"
//     .layout {
//       display: flex;
//       flex-direction: column;
//       gap: 2rem;
//     }

//     @media (min-width: 768px) {
//       .layout {
//         flex-direction: row;
//       }

//       .sidebar {
//         flex: 1;
//         max-width: 250px;
//       }

//       .main {
//         flex: 3;
//       }
//     }

//     .sidebar {
//       background-color: var(--muted-bg);
//       padding: 1rem;
//       border-radius: 0.5rem;
//     }
//     "#
//       }
//     }
// }
