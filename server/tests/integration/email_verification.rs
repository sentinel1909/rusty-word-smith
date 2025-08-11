// tests/integration/email_verification.rs

use crate::helpers::{TestApi, TestUser};
use pavex::http::StatusCode;
use sqlx::Row;

#[tokio::test]
async fn verify_endpoint_marks_user_verified_and_redirects() {
    let app = TestApi::spawn().await;
    let user = TestUser::unique();

    // Register user
    let r = app.post_register(&user).await;
    assert!(r.status().is_success());

    // Set a verification token directly (simulating email being sent)
    let token = uuid::Uuid::new_v4().to_string();
    sqlx::query("UPDATE users SET email_verification_token = $1, email_verification_expires_at = NOW() + interval '24 hours' WHERE email = $2")
        .bind(&token)
        .bind(&user.email)
        .execute(&app.api_db_pool)
        .await
        .expect("failed to set verification token");

    // Hit verify endpoint
    let r = app.get_verify(&token).await;
    assert!(r.status().is_redirection(), "verify should redirect on success, got {:?}", r.status());

    // Check DB updated
    let row = sqlx::query("SELECT email_verified, email_verification_token FROM users WHERE email = $1")
        .bind(&user.email)
        .fetch_one(&app.api_db_pool)
        .await
        .expect("user should exist");
    let verified: bool = row.get("email_verified");
    let token_db: Option<String> = row.get("email_verification_token");
    assert!(verified);
    assert!(token_db.is_none());
}

#[tokio::test]
async fn verify_endpoint_with_invalid_or_expired_token_shows_error() {
    let app = TestApi::spawn().await;
    let user = TestUser::unique();
    let r = app.post_register(&user).await;
    assert!(r.status().is_success());

    // Expired token
    let expired = uuid::Uuid::new_v4().to_string();
    sqlx::query("UPDATE users SET email_verification_token = $1, email_verification_expires_at = NOW() - interval '1 hour' WHERE email = $2")
        .bind(&expired)
        .bind(&user.email)
        .execute(&app.api_db_pool)
        .await
        .expect("failed to set verification token");

    let r = app.get_verify(&expired).await;
    assert_eq!(r.status(), StatusCode::BAD_REQUEST, "expired tokens should be rejected");

    // Invalid token
    let r = app.get_verify("not-a-real-token").await;
    assert_eq!(r.status(), StatusCode::BAD_REQUEST, "invalid tokens should be rejected");
}

#[tokio::test]
async fn resend_verification_sets_new_token_and_rate_limits() {
    let app = TestApi::spawn().await;
    let user = TestUser::unique();
    let r = app.post_register(&user).await;
    assert!(r.status().is_success());

    // First resend succeeds
    let r = app.post_resend_verification(&user.email).await;
    assert!(r.status().is_success());

    // Second immediate resend should be rate-limited (implementation may vary: 429 is common)
    let r = app.post_resend_verification(&user.email).await;
    assert!(r.status().is_client_error());
}

#[tokio::test]
async fn check_email_page_renders() {
    let app = TestApi::spawn().await;
    let r = app.get_check_email_page().await;
    assert!(r.status().is_success());
    let body = r.text().await.unwrap_or_default();
    assert!(body.to_lowercase().contains("check your email"));
}


