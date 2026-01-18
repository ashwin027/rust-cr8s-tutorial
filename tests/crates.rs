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
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    let crate1: Value = create_test_crate(&client, &rustacean);
    let crate2: Value = create_test_crate(&client, &rustacean);

    let response = client
        .get(format!("{}/crates", common::APP_HOST))
        .send()
        .unwrap();

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
    let client = common::get_client_with_logged_in_admin();
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
fn test_view_crates_not_found() {
    let client = common::get_client_with_logged_in_admin();

    let view_response = client
        .get(format!("{}/crates/{}", common::APP_HOST, 0))
        .send()
        .unwrap();
    assert_eq!(view_response.status(), StatusCode::NOT_FOUND);
    let view_crate: Value = view_response.json().unwrap();
    assert_eq!(view_crate.as_str(), Some("Not found"));
}

#[test]
fn test_view_crates() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    let a_crate: Value = create_test_crate(&client, &rustacean);

    let view_response = client
        .get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
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
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    let a_crate: Value = create_test_crate(&client, &rustacean);
    let rustacean2: Value = common::create_test_rustacean(&client);
    let updated_response = client
        .put(format!(
            "{}/crates/{}",
            common::APP_HOST,
            a_crate["id"]
        ))
        .json(&json!({
            "name": "Ashwin New Updated",
            "code": "test code updated",
            "rustacean_id": rustacean2["id"],
            "version": "v2",
            "description": "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus, tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. Nam quam nunc, blandit vel, luctus pulvinar, hendrerit id, lorem. Maecenas nec odio et ante tincidunt tempus. Donec vitae sapien ut libero venenatis faucibus. Nullam quis ante. Etiam sit amet orci eget eros faucibus tincidunt. Duis leo. Sed fringilla mauris sit amet nibh. Donec sodales sagittis magna. Sed consequat, leo eget bibendum sodales, augue velit cursus nunc,"
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
            "rustacean_id": rustacean2["id"],
            "version": "v2",
            "description": "Lorem ipsum dolor sit amet, consectetuer adipiscing elit. Aenean commodo ligula eget dolor. Aenean massa. Cum sociis natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Donec quam felis, ultricies nec, pellentesque eu, pretium quis, sem. Nulla consequat massa quis enim. Donec pede justo, fringilla vel, aliquet nec, vulputate eget, arcu. In enim justo, rhoncus ut, imperdiet a, venenatis vitae, justo. Nullam dictum felis eu pede mollis pretium. Integer tincidunt. Cras dapibus. Vivamus elementum semper nisi. Aenean vulputate eleifend tellus. Aenean leo ligula, porttitor eu, consequat vitae, eleifend ac, enim. Aliquam lorem ante, dapibus in, viverra quis, feugiat a, tellus. Phasellus viverra nulla ut metus varius laoreet. Quisque rutrum. Aenean imperdiet. Etiam ultricies nisi vel augue. Curabitur ullamcorper ultricies nisi. Nam eget dui. Etiam rhoncus. Maecenas tempus, tellus eget condimentum rhoncus, sem quam semper libero, sit amet adipiscing sem neque sed ipsum. Nam quam nunc, blandit vel, luctus pulvinar, hendrerit id, lorem. Maecenas nec odio et ante tincidunt tempus. Donec vitae sapien ut libero venenatis faucibus. Nullam quis ante. Etiam sit amet orci eget eros faucibus tincidunt. Duis leo. Sed fringilla mauris sit amet nibh. Donec sodales sagittis magna. Sed consequat, leo eget bibendum sodales, augue velit cursus nunc,",
            "created_at": a_crate["created_at"]
        })
    );

    // Cleanup
    delete_test_crate(&client, &a_crate);
    common::delete_test_rustacean(&client, &rustacean);
    common::delete_test_rustacean(&client, &rustacean2);
}

#[test]
fn test_delete_crates() {
    let client = common::get_client_with_logged_in_admin();
    let rustacean: Value = common::create_test_rustacean(&client);
    let a_crate: Value = create_test_crate(&client, &rustacean);

    let delete_response = client
        .delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(delete_response.status(), StatusCode::NO_CONTENT);
    common::delete_test_rustacean(&client, &rustacean);
}
