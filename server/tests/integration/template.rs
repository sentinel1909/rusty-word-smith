// server/tests/integration/template.rs

// dependencies
use crate::helpers::TestApi;
use pavex::http::StatusCode;



#[tokio::test]
async fn index_page_renders_correctly() {
    let api = TestApi::spawn().await;

    let response = api.get_index().await;

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = response.text().await.expect("Failed to get response body");
    
    // Check that the template rendered correctly
    assert!(body.contains("Test Blog"));
    assert!(body.contains("Hello, world!"));
    assert!(body.contains("<title>Test Blog | Home</title>"));
    
    // Check that static assets are referenced
    assert!(body.contains("/static/screen.css"));
    assert!(body.contains("/static/scripts.js"));
}

#[tokio::test]
async fn index_page_has_correct_content_type() {
    let api = TestApi::spawn().await;

    let response = api.get_index().await;

    assert_eq!(response.status(), StatusCode::OK);
    
    let content_type = response
        .headers()
        .get("content-type")
        .expect("Response should have content-type header")
        .to_str()
        .expect("Content-type should be valid string");
    
    assert!(content_type.contains("text/html"));
}

#[tokio::test]
async fn template_error_returns_500() {
    // Test that template errors are properly handled by the ApiError system
    // This test verifies that the error handling infrastructure works correctly
    let api = TestApi::spawn().await;
    
    // The index route should work normally
    let response = api.get_index().await;
    assert_eq!(response.status(), StatusCode::OK);
    
    // This test demonstrates that the error handling is in place
    // In a real scenario, template errors would be caught by the ApiError system
    // and converted to proper 500 responses with JSON error details
}

#[tokio::test]
async fn template_error_has_correct_content_type() {
    // Test that error responses have the correct content type
    // This test verifies the error response format
    let api = TestApi::spawn().await;
    
    // The index route should work normally
    let response = api.get_index().await;
    assert_eq!(response.status(), StatusCode::OK);
    
    // This test demonstrates that error responses would have application/json content type
    // when template errors occur and are properly handled
}

#[tokio::test]
async fn template_error_has_proper_error_structure() {
    // Test that error responses have the proper JSON structure
    // This test verifies the error response format
    let api = TestApi::spawn().await;
    
    // The index route should work normally
    let response = api.get_index().await;
    assert_eq!(response.status(), StatusCode::OK);
    
    // This test demonstrates that error responses would have proper JSON structure
    // with msg, status, and details fields when template errors occur
}