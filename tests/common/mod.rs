use reqwest::{blocking::Client, StatusCode};
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