// server/tests/integration/admin.rs

// dependencies
use crate::helpers::{TestApi, TestUser};

#[tokio::test]
async fn admin_can_access_admin_route() {
    let app = TestApi::spawn().await;

    // Register a user
    let user = TestUser::new("adminuser", "admin@example.com", "P@ssw0rd!", "Admin User");
    let r = app.post_register(&user).await;
    assert!(r.status().is_success(), "register should succeed");

    // Promote to admin directly in DB (pre-login)
    sqlx::query("UPDATE users SET role = 'admin' WHERE username = $1")
        .bind(&user.username)
        .execute(&app.api_db_pool)
        .await
        .expect("failed to promote user to admin");

    // Login
    let r = app.post_login(&user.username, &user.password).await;
    assert!(r.status().is_success(), "login should succeed");

    // Access /admin
    let r = app
        .api_client
        .get(format!("{}/admin", app.api_address))
        .send()
        .await
        .expect("Failed to execute GET /admin");

    assert!(
        r.status().is_success(),
        "admin should be able to access /admin, got {}",
        r.status()
    );
}

#[tokio::test]
async fn unauthenticated_access_to_admin_returns_401() {
    let app = TestApi::spawn().await;

    let r = app
        .api_client
        .get(format!("{}/admin", app.api_address))
        .send()
        .await
        .expect("Failed to execute GET /admin");

    assert_eq!(
        r.status().as_u16(),
        401,
        "unauthenticated should get 401, got {}",
        r.status()
    );
}

#[tokio::test]
async fn non_admin_access_to_admin_returns_403() {
    let app = TestApi::spawn().await;

    // Register default (subscriber) user
    let user = TestUser::new("bob", "bob@example.com", "P@ssw0rd!", "Bob");
    let r = app.post_register(&user).await;
    assert!(r.status().is_success(), "register should succeed");

    // Login
    let r = app.post_login(&user.username, &user.password).await;
    assert!(r.status().is_success(), "login should succeed");

    // Access /admin as non-admin
    let r = app
        .api_client
        .get(format!("{}/admin", app.api_address))
        .send()
        .await
        .expect("Failed to execute GET /admin");

    assert_eq!(
        r.status().as_u16(),
        403,
        "non-admin should get 403, got {}",
        r.status()
    );
}
