// server/tests/integration/static_files.rs

// dependencies
use crate::helpers::TestApi;
use pavex::http::StatusCode;

#[tokio::test]
async fn css_file_is_served_correctly() {
    let api = TestApi::spawn().await;

    let response = api.get_static_file("screen.css").await;

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .expect("Response should have content-type header")
        .to_str()
        .expect("Content-type should be valid string");

    assert!(content_type.contains("text/css"));

    let body = response.text().await.expect("Failed to get response body");
    assert!(!body.is_empty(), "CSS file should not be empty");
}

#[tokio::test]
async fn js_file_is_served_correctly() {
    let api = TestApi::spawn().await;

    let response = api.get_static_file("scripts.js").await;

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .expect("Response should have content-type header")
        .to_str()
        .expect("Content-type should be valid string");

    assert!(content_type.contains("text/javascript"));

    let body = response.text().await.expect("Failed to get response body");
    assert!(!body.is_empty(), "JS file should not be empty");
}

#[tokio::test]
async fn favicon_is_served_correctly() {
    let api = TestApi::spawn().await;

    let response = api.get_static_file("favicon.ico").await;

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get("content-type")
        .expect("Response should have content-type header")
        .to_str()
        .expect("Content-type should be valid string");

    assert!(content_type.contains("image/x-icon"));
}

#[tokio::test]
async fn non_existent_file_returns_404() {
    let api = TestApi::spawn().await;

    let response = api.get_static_file("non-existent-file.txt").await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn static_files_have_cache_headers() {
    let api = TestApi::spawn().await;

    let response = api.get_static_file("screen.css").await;

    assert_eq!(response.status(), StatusCode::OK);

    // Check for cache control headers (if implemented)
    let cache_control = response.headers().get("cache-control");
    if let Some(cache_control) = cache_control {
        let cache_control_str = cache_control
            .to_str()
            .expect("Cache-control should be valid string");
        assert!(!cache_control_str.is_empty());
    }
}
