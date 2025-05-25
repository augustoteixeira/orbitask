use maud::{html, Markup};

pub fn header() -> Markup {
    html! {
      header {
        nav {
          h1 { "Orbitask" }
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
           href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css";
    }
}

pub fn sidebar_style() -> Markup {
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
