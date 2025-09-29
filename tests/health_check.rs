use std::net::TcpListener;

use serde::Serialize;

#[tokio::test]
async fn health_check_works() {
    let addr = spawn_app().await;
    let addr = format!("{}/health_check", addr);
    println!("addr: {}", addr);
    let client = reqwest::Client::new();
    let response = client
        .get(&addr)
        .send()
        .await
        .expect("Failed to execute request!");
    assert_eq!(response.status(), 200);
}

#[derive(serde::Deserialize, Serialize)]
struct SubscribeBodyTest {
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let addr = spawn_app().await;
    let addr = format!("{}/subscriptions", addr);
    let client = reqwest::Client::new();
    let body = SubscribeBodyTest {
        name: Some("codecow".to_string()),
        email: Some("codedouzhenbo@gmail.com".to_string()),
    };
    let response = client
        .post(&addr)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("Failed to execute request!");
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let addr = spawn_app().await;
    let addr = format!("{}/subscriptions", addr);
    let client = reqwest::Client::new();
    let datas = vec![
        (
            SubscribeBodyTest {
                name: Some("codecow".to_string()),
                email: None,
            },
            "missing the email",
        ),
        (
            SubscribeBodyTest {
                name: None,
                email: Some("codedouzhenbo@gmail.com".to_string()),
            },
            "missing the name",
        ),
        (
            SubscribeBodyTest {
                name: None,
                email: None,
            },
            "missing both name and email",
        ),
    ];
    for (body, error_message) in datas {
        let response = client
            .post(&addr)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .expect("Failed to execute request!");
        assert_eq!(
            response.status(),
            400,
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let addr = format!("http://{}", listener.local_addr().unwrap());
    let server = zero2prod::startup::run(listener).await.unwrap();
    tokio::spawn(server);
    addr
}
