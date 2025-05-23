use maud::{html, Markup};

use crate::db_manage::State;

pub fn states_grid(states: Vec<State>) -> Markup {
    html! {
      section style=r#"
      display: grid;
      gap: 1rem;
      grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    "# {
        @for state in states {
          article style=r#"
          padding: 1rem;
          border: 1px solid var(--muted-border);
          border-radius: 0.5rem;
        "# {
            h3 { (state.name) }

            @if state.is_finished {
              p style="font-size: 0.75em; color: var(--muted-color);" {
                "Finished state"
              }
            } @else {
              p style="font-size: 0.75em; color: var(--muted-color);" {
                "Active state"
              }
            }
          }
        }
      }
    }
}
