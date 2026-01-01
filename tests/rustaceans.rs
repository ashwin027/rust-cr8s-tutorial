use reqwest::{blocking::Client, StatusCode};
use rocket::serde::json::{serde_json::json, Value};

fn create_test_rustacean(client: &Client) -> Value {
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

#[test]
fn test_get_rustaceans() {
    let client = Client::new();
    let rustacean1: Value = create_test_rustacean(&client);
    let rustacean2: Value = create_test_rustacean(&client);

    let response = client
        .get("http://127.0.0.1:8000/rustaceans")
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();

    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));
}

#[test]
fn test_create_rustaceans() {
    let client = Client::new();
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
    
    let rustacean: Value = response.json().unwrap();
    assert_eq!(
        rustacean,
        json!({
            "id": rustacean["id"],
            "name": "Ashwin New",
            "email": "ashwin@test.com",
            "created_at": rustacean["created_at"]
        })
    );
}

#[test]
fn test_view_rustaceans() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let view_response = client
        .get(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(view_response.status(), StatusCode::OK);
    let view_rustacean: Value = view_response.json().unwrap();
    assert_eq!(
        view_rustacean,
        json!({
            "id": view_rustacean["id"],
            "name": "Ashwin New",
            "email": "ashwin@test.com",
            "created_at": view_rustacean["created_at"]
        })
    );
}

#[test]
fn test_update_rustaceans() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let updated_response = client
        .put(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .json(&json!({
            "name": "Ashwin Newly updated",
            "email": "ashwin@updated.com"
        }))
        .send()
        .unwrap();
    assert_eq!(updated_response.status(), StatusCode::OK);
    let updated_rustacean: Value = updated_response.json().unwrap();
    assert_eq!(
        updated_rustacean,
        json!({
            "id": updated_rustacean["id"],
            "name": "Ashwin Newly updated",
            "email": "ashwin@updated.com",
            "created_at": updated_rustacean["created_at"]
        })
    );
}

#[test]
fn test_delete_rustaceans() {
    let client = Client::new();
    let rustacean: Value = create_test_rustacean(&client);

    let delete_response = client
        .delete(format!(
            "http://127.0.0.1:8000/rustaceans/{}",
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}
