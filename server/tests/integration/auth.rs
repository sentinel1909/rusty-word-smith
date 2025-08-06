// tests/api/integration/auth.rs

// dependencies
use crate::helpers::{TestApi, TestUser};
use pavex::http::StatusCode;
use serde_json::Value;
use sqlx::Row;

// test which exercises the user registration endpoint
#[tokio::test]
async fn register_user_returns_200_ok_and_user_record() {
    // Given
    let api = TestApi::spawn().await;
    let test_user = TestUser::unique();

    // When
    let response = api.post_register(&test_user).await;

    // Then
    let status = response.status();
    if status != StatusCode::OK {
        let response_body = response.text().await.expect("Failed to get response body");
        println!("Response status: {}", status);
        println!("Response body: {}", response_body);
        panic!("Expected OK status but got {}", status);
    }

    let response_body: Value = response
        .json()
        .await
        .expect("Failed to parse response JSON");

    // Verify the response contains the expected user data
    assert_eq!(response_body["username"], test_user.username);
    assert_eq!(response_body["email"], test_user.email);
    assert_eq!(response_body["display_name"], test_user.display_name);

    // Verify required fields are present
    assert!(response_body.get("id").is_some());
    assert!(response_body.get("role").is_some());
    assert!(response_body.get("is_active").is_some());
    assert!(response_body.get("email_verified").is_some());
    assert!(response_body.get("created_at").is_some());

    // Verify password is not returned in response
    assert!(response_body.get("password").is_none());

    // Verify the user was actually persisted to the database
    let user_id = response_body["id"].as_str().expect("User ID should be present");
    let user_uuid = uuid::Uuid::parse_str(user_id).expect("User ID should be a valid UUID");
    
    // Query the database directly to verify persistence
    let db_user = sqlx::query(
        "SELECT username, email, display_name, role::text, is_active, email_verified FROM users WHERE id = $1"
    )
    .bind(user_uuid)
    .fetch_one(&api._api_db_pool)
    .await
    .expect("User should exist in database");

    // Verify the database record matches the request data
    assert_eq!(db_user.get::<String, _>("username"), test_user.username);
    assert_eq!(db_user.get::<String, _>("email"), test_user.email);
    assert_eq!(db_user.get::<String, _>("display_name"), test_user.display_name);
    
    // Verify default values are set correctly
    assert_eq!(db_user.get::<String, _>("role"), "subscriber");
    assert_eq!(db_user.get::<bool, _>("is_active"), true);
    assert_eq!(db_user.get::<bool, _>("email_verified"), false);
}
