mod common;
mod integration;

use rocket::tokio;

#[tokio::test]
async fn test_root_notes_shows_test_note() {
    let _ = common::prepare_test_db().await;

    let rocket = integration::spawn_test_rocket().await;
    let client = rocket::local::asynchronous::Client::untracked(rocket)
        .await
        .expect("valid rocket instance");

    let response = client.get("/login").dispatch().await;
    println!("{:?}", response);
    assert_eq!(response.status(), rocket::http::Status::Ok);
    let body = response.into_string().await.unwrap();
    println!("{:?}", body);
    assert!(
        body.contains("Test note"),
        "Response body did not contain expected note title: {body}"
    );
}
