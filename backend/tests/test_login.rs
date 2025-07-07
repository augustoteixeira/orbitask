mod common;
mod integration;

use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use rocket::{
    http::ContentType, http::Status, local::asynchronous::Client, tokio,
};

const LOCALHOST: SocketAddrV4 =
    std::net::SocketAddrV4::new(std::net::Ipv4Addr::LOCALHOST, 8000);

#[tokio::test]
async fn test_unauthorized() {
    let _ = common::prepare_test_db().await;
    let rocket = integration::spawn_test_rocket().await;
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
    let rocket = integration::spawn_test_rocket().await;
    let client = rocket::local::asynchronous::Client::untracked(rocket)
        .await
        .expect("valid rocket instance");
    let response = client.get("/login").dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}

#[tokio::test]
async fn test_login_success() {
    let _ = common::prepare_test_db().await;
    let rocket = integration::spawn_test_rocket().await;
    let client = rocket::local::asynchronous::Client::untracked(rocket)
        .await
        .expect("valid rocket instance");
    let mut request = client
        .post("/login")
        .header(rocket::http::ContentType::Form)
        .body("password=123")
        .remote(LOCALHOST.into());
    let response = request.dispatch().await;
    assert_eq!(response.status(), Status::SeeOther);
    let location = response.headers().get_one("Location");
    assert_eq!(location, Some("/"));
}

pub async fn login_as_test_user() -> Client {
    let rocket = integration::spawn_test_rocket().await;
    let client = Client::tracked(rocket)
        .await
        .expect("valid rocket instance");
    {
        let response = client
            .post("/login")
            .header(ContentType::Form)
            .body("password=123")
            .remote(LOCALHOST.into())
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::SeeOther, "Login failed");
    }
    client
}

#[tokio::test]
async fn test_login_then_root() {
    let _ = common::prepare_test_db().await;
    let client = login_as_test_user().await;
    let response = client.get("/").remote(LOCALHOST.into()).dispatch().await;
    assert_eq!(response.status(), Status::Ok);
}
