// tests/integration/auth.rs

// dependencies
use crate::helpers::{TestApi, TestUser};
use reqwest::header;

#[tokio::test]
async fn login_fails_until_verified_then_succeeds() {
    let app = TestApi::spawn().await;

    // seed user via /auth/register
    let user = TestUser::new("alice", "alice@example.com", "P@ssw0rd!", "alice");
    let r = app.post_register(&user).await;
    assert!(r.status().is_success());

    // login should fail before verification
    let r = app.post_login("alice", "P@ssw0rd!").await;
    assert!(r.status().is_client_error(), "login should fail while unverified");

    // whoami with persisted cookie
    // Manually mark user as verified to simulate clicking the link (until verify route is implemented)
    sqlx::query("UPDATE users SET email_verified = true WHERE email = $1")
        .bind(&user.email)
        .execute(&app.api_db_pool)
        .await
        .expect("failed to set email_verified=true");

    // login should now succeed
    let r = app.post_login("alice", "P@ssw0rd!").await;
    assert!(r.status().is_success(), "login should succeed after verification");

    // whoami with persisted cookie
    let r = app.get_whoami().await;
    assert!(r.status().is_success());
    let body = r.text().await.unwrap();
    assert!(body.contains("\"username\":\"alice\""));
}

#[tokio::test]
async fn logout_invalidates_session_and_whoami_returns_401() {
    // Arrange
    let app = TestApi::spawn().await;

    // Seed a user via /auth/register
    let user = TestUser::new("alice", "alice@example.com", "P@ssw0rd!", "alice");
    let r = app.post_register(&user).await;
    assert!(r.status().is_success(), "register should succeed");

    // Mark as verified to allow login
    sqlx::query("UPDATE users SET email_verified = true WHERE username = $1")
        .bind("alice")
        .execute(&app.api_db_pool)
        .await
        .expect("failed to set email_verified");

    // Log in
    let r = app.post_login("alice", "P@ssw0rd!").await;
    assert!(r.status().is_success(), "login should succeed");

    // Sanity: cookie is set
    let set_cookie = r.headers().get_all(header::SET_COOKIE);
    assert!(set_cookie.iter().next().is_some(), "login must set a session cookie");

    // Sanity: whoami works while logged in
    let r = app.get_whoami().await;
    assert!(r.status().is_success(), "whoami should succeed while logged in");
    let body = r.text().await.unwrap();
    assert!(body.contains("\"username\":\"alice\""));

    // Act: logout
    let r = app.post_logout().await;
    // You may return 204 No Content or 200 OK; accept either for now.
    assert!(
        r.status().as_u16() == 204 || r.status().as_u16() == 200,
        "logout should succeed (200/204), got {}",
        r.status()
    );

    // Optional: cookie-clear hint (implementation-dependent)
    // Not all setups send an explicit clearing cookie; if yours does, this will pass.
    let _ = r
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .any(|v| {
            let s = v.to_str().unwrap_or("");
            s.starts_with("sid=") && (s.contains("Max-Age=0") || s.to_lowercase().contains("expires="))
        });
    // It's fine if `cleared` is false—some stores rely on server-side invalidation only.

    // Assert: whoami now fails (session invalidated)
    let r = app.get_whoami().await;
    assert_eq!(
        r.status().as_u16(),
        401,
        "whoami should return 401 after logout; body={:?}",
        r.text().await.ok()
    );
}

