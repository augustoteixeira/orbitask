use crate::db_manage::states::{delete_state, move_state, rename_state};
use crate::Db;

use rocket::form::Form;
use rocket::response::{Flash, Redirect};
use rocket_db_pools::Connection;

use super::Authenticated;

#[derive(FromForm)]
pub struct MoveStateForm {
    old_position: u64,
    new_position: u64,
}

#[post("/states/<state_id>/move", data = "<form>")]
pub async fn move_state_api(
    _auth: Authenticated,
    mut db: Connection<Db>,
    state_id: i64,
    form: Form<MoveStateForm>,
) -> Result<Redirect, Flash<Redirect>> {
    let form = form.into_inner();

    match move_state(&mut db, state_id, form.old_position, form.new_position)
        .await
    {
        Ok(Some(state)) => {
            // Redirect to the board's page to reflect updated state
            Err(Flash::success(
                Redirect::to(format!("/boards/{}", state.board_id)),
                "State moved",
            ))
        }
        Ok(None) => {
            Err(Flash::error(Redirect::to("/boards"), "State not found"))
        }
        Err(e) => {
            eprintln!("Error moving state: {e}");
            Err(Flash::error(
                Redirect::to("/boards"),
                "Error moving state: {e}",
            ))
        }
    }
}

#[post("/states/<state_id>/delete")]
pub async fn delete_state_api(
    _auth: Authenticated,
    mut db: Connection<Db>,
    state_id: i64,
) -> Result<Redirect, Flash<Redirect>> {
    match delete_state(&mut db, state_id).await {
        Ok(Some(state)) => {
            // Redirect to the board's page to reflect updated state
            Err(Flash::success(
                Redirect::to(format!("/boards/{}", state.board_id)),
                "State deleted",
            ))
        }
        Ok(None) => {
            Err(Flash::error(Redirect::to("/boards"), "State not found"))
        }
        Err(e) => {
            eprintln!("Error moving state: {e}");
            Err(Flash::error(
                Redirect::to("/boards"),
                "Error moving state: {e}",
            ))
        }
    }
}

#[derive(FromForm)]
pub struct RenameStateForm {
    new_name: String,
}

#[post("/states/<state_id>/rename", data = "<form>")]
pub async fn rename_state_api(
    _auth: Authenticated,
    mut db: Connection<Db>,
    state_id: i64,
    form: Form<RenameStateForm>,
) -> Result<Redirect, Flash<Redirect>> {
    let form = form.into_inner();
    match rename_state(&mut db, state_id, form.new_name).await {
        Ok(Some(state)) => {
            // Redirect to the board's page to reflect updated state
            Err(Flash::success(
                Redirect::to(format!("/boards/{}", state.board_id)),
                "State deleted",
            ))
        }
        Ok(None) => {
            Err(Flash::error(Redirect::to("/boards"), "State not found"))
        }
        Err(e) => {
            eprintln!("Error moving state: {e}");
            Err(Flash::error(
                Redirect::to("/boards"),
                "Error moving state: {e}",
            ))
        }
    }
}
