mod common;
use rocket::{http::Status, tokio};

use common::LOCALHOST;

#[tokio::test]
async fn test_root() {
    let _ = common::prepare_test_db().await;
    let client = common::login_as_test_user().await;
    let response = client.get("/").remote(LOCALHOST.into()).dispatch().await;
    let contents = response
        .into_string()
        .await
        .expect("Could not get contents");
    // TODO Check children
}
