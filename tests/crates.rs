use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};
mod common;

fn delete_test_crate(client: &Client, a_crate: &Value) {
    let delete_response = client
        .delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}

fn create_test_crate(client: &Client, rustacean: &Value) -> Value {
    let request = json!({
        "name": "Ashwin New",
        "code": "test code",
        "rustacean_id": rustacean["id"],
        "version": "v1",
        "description": "test description"
    });
    let response = client
        .post(format!("{}/crates", common::APP_HOST))
        .json(&request)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    response.json().unwrap()
}

#[test]
fn test_get_crates() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    let crate1: Value = create_test_crate(&client, &rustacean);
    let crate2: Value = create_test_crate(&client, &rustacean);

    let response = client.get(format!("{}/crates", common::APP_HOST)).send().unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();

    assert!(json.as_array().unwrap().contains(&crate1));
    assert!(json.as_array().unwrap().contains(&crate2));

    // Cleanup
    delete_test_crate(&client, &crate1);
    delete_test_crate(&client, &crate2);
    common::delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_create_crates() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    let request = json!({
        "name": "Ashwin New",
        "code": "test code",
        "rustacean_id": rustacean["id"],
        "version": "v1",
        "description": "test description"
    });
    let response = client
        .post(format!("{}/crates", common::APP_HOST))
        .json(&request)
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let a_crate: Value = response.json().unwrap();
    assert_eq!(
        a_crate,
        json!({
            "id": a_crate["id"],
            "name": "Ashwin New",
            "code": "test code",
            "rustacean_id": rustacean["id"],
            "version": "v1",
            "description": "test description",
            "created_at": a_crate["created_at"]
        })
    );

    // Cleanup
    delete_test_crate(&client, &a_crate);
    common::delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_view_crates() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    let a_crate: Value = create_test_crate(&client, &rustacean);

    let view_response = client
        .get(format!(
            "{}/crates/{}",
            common::APP_HOST,
            a_crate["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(view_response.status(), StatusCode::OK);
    let view_crate: Value = view_response.json().unwrap();
    assert_eq!(
        view_crate,
        json!({
            "id": a_crate["id"],
            "name": "Ashwin New",
            "code": "test code",
            "rustacean_id": rustacean["id"],
            "version": "v1",
            "description": "test description",
            "created_at": a_crate["created_at"]
        })
    );

    // Cleanup
    delete_test_crate(&client, &a_crate);
    common::delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_update_crates() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    let a_crate: Value = create_test_crate(&client, &rustacean);

    let updated_response = client
        .put(format!(
            "{}/crates/{}",
            common::APP_HOST,
            a_crate["id"]
        ))
        .json(&json!({
            "name": "Ashwin New Updated",
            "code": "test code updated",
            "rustacean_id": rustacean["id"],
            "version": "v2",
            "description": "test description Updated"
        }))
        .send()
        .unwrap();
    assert_eq!(updated_response.status(), StatusCode::OK);
    let updated_crate: Value = updated_response.json().unwrap();
    assert_eq!(
        updated_crate,
        json!({
            "id": a_crate["id"],
            "name": "Ashwin New Updated",
            "code": "test code updated",
            "rustacean_id": rustacean["id"],
            "version": "v2",
            "description": "test description Updated",
            "created_at": a_crate["created_at"]
        })
    );

    // Cleanup
    delete_test_crate(&client, &a_crate);
    common::delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_delete_crates() {
    let client = Client::new();
    let rustacean: Value = common::create_test_rustacean(&client);
    let a_crate: Value = create_test_crate(&client, &rustacean);

    let delete_response = client
        .delete(format!(
            "{}/crates/{}",
            common::APP_HOST,
            a_crate["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
    common::delete_test_rustacean(&client, &rustacean);
}
