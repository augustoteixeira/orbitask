use maud::{html, Markup};

use crate::db_manage::{get_notes_for_state, get_state, Note, State};

use crate::frontend::note::note_list;
use crate::sqlx::FromRow;
use crate::Db;
use rocket_db_pools::Connection;

#[derive(Debug, FromRow)]
pub struct StateView {
    pub state: State,
    pub notes: Vec<Note>,
}

pub async fn get_state_view(
    db: &mut Connection<Db>,
    id: i64,
) -> Result<StateView, sqlx::Error> {
    let state = get_state(db, id).await.unwrap().unwrap();
    let notes = get_notes_for_state(db, state.id).await?;
    Ok(StateView { state, notes })
}

pub fn state_render(state_view: StateView) -> Markup {
    html! {
        article style=r#"
        padding: 1rem;
        border: 1px solid var(--muted-border);
        border-radius: 0.5rem;
        "# {
        h3 { (state_view.state.name) }

        @if state_view.state.is_finished {
          p style="font-size: 0.75em; color: var(--muted-color);" {
            "Finished state"
          }
        } @else {
          p style="font-size: 0.75em; color: var(--muted-color);" {
            "Active state"
          }
        }
        (note_list(state_view.notes))
      }
    }
}

pub fn states_grid(states: Vec<StateView>) -> Markup {
    html! {
      section style=r#"
      display: grid;
      gap: 1rem;
      grid-template-columns: repeat(auto-fit, minmax(160px, 1fr));
    "# {
        @for state in states {
            (state_render(state))
        }
      }
    }
}
