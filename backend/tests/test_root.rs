mod common;
use backend::frontend::view::ViewState;
use rocket::{http::ContentType, http::Status, tokio};

use common::LOCALHOST;

#[tokio::test]
async fn test_root() {
    let _ = common::prepare_test_db().await;
    let client = common::login_as_test_user().await;
    let response = client
        .get("/")
        .header(ContentType::JSON)
        .remote(LOCALHOST.into())
        .dispatch()
        .await;
    let contents = response
        .into_string()
        .await
        .expect("Could not get contents");
    let view: ViewState = serde_json::from_str(contents.as_str())
        .expect("Could not parse contents");
    let root = match view {
        ViewState::Root(r) => r,
        _ => {
            panic!("View was not of type root");
        }
    };
    assert!(
        root.into_iter()
            .any(|note| note.title == "Main Project".to_string()),
        "No note named Main Project"
    );
}
