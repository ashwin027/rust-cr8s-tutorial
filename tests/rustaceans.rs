use reqwest::{blocking::Client, StatusCode};
use serde_json::{json, Value};

use crate::common::delete_test_rustacean;
use crate::common::create_test_rustacean;
mod common;

#[test]
fn test_get_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean1: Value = create_test_rustacean(&client);
    let rustacean2: Value = create_test_rustacean(&client);

    let response = client
        .get(format!("{}/rustaceans", common::APP_HOST))
        .send()
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();

    assert!(json.as_array().unwrap().contains(&rustacean1));
    assert!(json.as_array().unwrap().contains(&rustacean2));

    // Cleanup
    delete_test_rustacean(&client, &rustacean1);
    delete_test_rustacean(&client, &rustacean2);
}

#[test]
fn test_create_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let request = json!({
        "name": "Ashwin New",
        "email": "ashwin@test.com"
    });
    let response = client
        .post(format!("{}/rustaceans", common::APP_HOST))
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

    // Cleanup
    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_view_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = create_test_rustacean(&client);

    let view_response = client
        .get(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
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

    // Cleanup
    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_view_rustacean_not_found() {
    let client = common::get_client_with_logged_in_admin();

    let view_response = client
        .get(format!("{}/rustaceans/{}", common::APP_HOST, 0))
        .send()
        .unwrap();
    assert_eq!(view_response.status(), StatusCode::NOT_FOUND);
    let view_crate: Value = view_response.json().unwrap();
    assert_eq!(view_crate.as_str(), Some("Not found"));
}

#[test]
fn test_update_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);

    let updated_response = client
        .put(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
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

    // Cleanup
    delete_test_rustacean(&client, &rustacean);
}

#[test]
fn test_delete_rustaceans() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);

    let delete_response = client
        .delete(format!(
            "{}/rustaceans/{}",
            common::APP_HOST,
            rustacean["id"]
        ))
        .send()
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
}
