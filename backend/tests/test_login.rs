mod common;
mod integration;

use common::{spawn_test_rocket, LOCALHOST};
use rocket::{http::Status, tokio};

#[tokio::test]
async fn test_unauthorized() {
    let _ = common::prepare_test_db().await;
    let rocket = spawn_test_rocket().await;
    let client = rocket::local::asynchronous::Client::untracked(rocket)
        .await
        .expect("valid rocket instance");
    let response = client.get("/").dispatch().await;
    assert_eq!(response.status(), Status::SeeOther);
    // TODO check if it redirects correctly.
}

#[tokio::test]
async fn test_login_page() {
    let _ = common::prepare_test_db().await;
    let rocket = spawn_test_rocket().await;
    let client = rocket::local::asynchronous::Client::untracked(rocket)
        .await
        .expect("valid rocket instance");
    let response = client.get("/login").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[tokio::test]
async fn test_login_success() {
    let _ = common::prepare_test_db().await;
    let rocket = spawn_test_rocket().await;
    let client = rocket::local::asynchronous::Client::untracked(rocket)
        .await
        .expect("valid rocket instance");
    let request = client
        .post("/login")
        .header(rocket::http::ContentType::Form)
        .body("password=123")
        .remote(LOCALHOST.into());
    let response = request.dispatch().await;
    assert_eq!(response.status(), Status::SeeOther);
    let location = response.headers().get_one("Location");
    assert_eq!(location, Some("/"));
}

#[tokio::test]
async fn test_login_then_root() {
    let _ = common::prepare_test_db().await;
    let client = common::login_as_test_user().await;
    let response = client.get("/").remote(LOCALHOST.into()).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
