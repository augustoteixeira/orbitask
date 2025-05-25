use crate::db_manage::states::move_state;
use crate::Db;

use crate::api::require_auth;
use rocket::form::Form;
use rocket::response::Redirect;
use rocket_db_pools::Connection;

use super::User;

#[derive(FromForm)]
pub struct MoveStateForm {
    old_position: u64,
    new_position: u64,
}

#[post("/states/<state_id>/move", data = "<form>")]
pub async fn move_state_api(
    user: Option<User>,
    mut db: Connection<Db>,
    state_id: i64,
    form: Form<MoveStateForm>,
) -> Result<Redirect, Redirect> {
    require_auth(user)?;

    let form = form.into_inner();

    match move_state(&mut db, state_id, form.old_position, form.new_position)
        .await
    {
        Ok(Some(state)) => {
            // Redirect to the board's page to reflect updated state
            Ok(Redirect::to(format!("/boards/{}", state.board_id)))
        }
        Ok(None) => {
            // State not found
            Err(Redirect::to("/boards")) // Could add flash message here
        }
        Err(e) => {
            eprintln!("Error moving state: {e}");
            Err(Redirect::to("/boards"))
        }
    }
}
