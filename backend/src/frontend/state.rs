use maud::{html, Markup};

use crate::db_manage::{
    get_notes_for_state, get_state, get_states_for_board, Note, State,
};

use crate::frontend::note::note_list;
use crate::sqlx::FromRow;
use crate::Db;
use rocket_db_pools::Connection;

#[derive(Debug, FromRow)]
pub struct StateView {
    pub state: State,
    pub notes: Vec<Note>,
    pub max_position: i64,
}

pub async fn get_state_view(
    db: &mut Connection<Db>,
    id: i64,
) -> Result<StateView, sqlx::Error> {
    let state = get_state(db, id).await.unwrap().unwrap();
    let notes = get_notes_for_state(db, state.id).await?;
    let states = get_states_for_board(db, state.board_id).await.unwrap();
    Ok(StateView {
        state,
        notes,
        max_position: states.len() as i64,
    })
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

pub fn state_render(state_view: StateView) -> Markup {
    let pos = state_view.state.position;
    let id = state_view.state.id;

    html! {
      article style=r#"
      padding: 1rem;
      border: 1px solid var(--muted-border);
      border-radius: 0.5rem;
    "# {
        h3 style="margin-bottom: 0.5rem" { (state_view.state.name) }

        @if state_view.state.is_finished {
          p style="font-size: 0.75em; color: var(--muted-color);" {
            "Finished state"
          }
        } @else {
          p style="font-size: 0.75em; color: var(--muted-color);" {
            "Active state"
          }
        }

        // Move buttons
        form method="post" action={(format!("/states/{}/move", id))} style="display: flex; gap: 0.5rem; align-items: center; margin: 0.5rem 0;" {
          input type="hidden" name="old_position" value={(pos)};

          @if pos > 0 {
            button
              type="submit"
              name="new_position"
              value={(pos - 1)}
              title="Move Left"
              style="padding: 0.2em 0.5em; font-size: 1em;"
            { "←" }
          }

          @if pos < state_view.max_position - 1 {
            button
              type="submit"
              name="new_position"
              value={(pos + 1)}
              title="Move Right"
              style="padding: 0.2em 0.5em; font-size: 1em;"
            { "→" }
          }
        }

        nav style="margin-bottom: 1rem; margin-top: 1rem" {
          a href={(format!("/boards/{}/{}", state_view.state.board_id,
                           state_view.state.id) + "/new_note")}
            role="button" {
            "New note"
          }
        }

        (note_list(state_view.notes))
      }
    }
}
