use std::process::Command;

use reqwest::{StatusCode, blocking::{Client, ClientBuilder}, header};
use serde_json::{json, Value};

pub static APP_HOST: &'static str = "http://127.0.0.1:8000";
pub fn delete_test_rustacean(client: &Client, rustacean: &Value){
    let delete_response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}

pub fn create_test_rustacean(client: &Client) -> Value {
    let request = json!({
        "name": "Ashwin New",
        "email": "ashwin@test.com"
    });
    let response = client
        .post("http://127.0.0.1:8000/rustaceans")
        .json(&request)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

pub fn get_client_with_logged_in_admin() -> Client {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--bin")
        .arg("cli")
        .arg("users")
        .arg("create")
        .arg("test_admin")
        .arg("1234")
        .arg("admin")
        .output()
        .unwrap();

    println!("{:?}", output);

    let client = Client::new();

    let request = json!({
        "username": "test_admin",
        "password": "1234"
    });
    let response = client
        .post(format!("{}/login", APP_HOST))
        .json(&request)
        .send()
        .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

    let json: Value = response.json().unwrap();
    assert!(json.get("token").is_some());

    let header_value = format!("Bearer {}", json["token"].as_str().unwrap());
    let mut headers = header::HeaderMap::new();
    let auth_value = header::HeaderValue::from_str(&header_value);
    headers.insert(header::AUTHORIZATION, auth_value.unwrap());
    let client = ClientBuilder::new()
    .default_headers(headers)
    .build();

    return client.unwrap();
}