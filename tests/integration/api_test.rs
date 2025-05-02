//! Integration tests for API endpoints

use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware,
    routing::{get, post, put},
    Router,
};
use kosync::{api, db::DB, defs::ProgressState};
use serde_json::{json, Value};
use std::path::PathBuf;
use tempfile::tempdir;
use tower::ServiceExt;

// Import test utilities
use crate::common::{
    auth_helpers::add_auth_headers,
    fixtures::{default_test_progress_state, default_test_user},
};

// Helper function to create a test app with a temporary database
async fn create_test_app() -> (Router, PathBuf) {
    // Create a temporary directory for the test database
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_db");
    
    // Create a new database
    let db = DB::new(&db_path).expect("Failed to create database");
    
    // Create the router
    let router = Router::new()
        .route("/users/create", post(api::create_user))
        .route("/healthcheck", get(api::healthcheck))
        .merge(
            Router::new()
                .route("/users/auth", get(api::auth_user))
                .route("/syncs/progress", put(api::update_progress))
                .route("/syncs/progress/:doc", get(api::get_progress))
                .layer(middleware::from_fn_with_state(db.clone(), api::auth)),
        )
        .with_state(db);
    
    (router, db_path.to_path_buf())
}

#[tokio::test]
async fn test_healthcheck() {
    // Create a test app
    let (app, _) = create_test_app().await;
    
    // Create a request to the healthcheck endpoint
    let response = app
        .oneshot(Request::builder().uri("/healthcheck").body(Body::empty()).unwrap())
        .await
        .unwrap();
    
    // Check the response
    assert_eq!(response.status(), StatusCode::OK);
    
    // Parse the response body
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    
    // Check the response body
    assert_eq!(body, json!({"state": "OK"}));
}

#[tokio::test]
async fn test_create_user() {
    // Create a test app
    let (app, _) = create_test_app().await;
    
    // Create a user
    let (username, password) = default_test_user();
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/create")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": username,
                        "password": password,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Check the response
    assert_eq!(response.status(), StatusCode::CREATED);
    
    // Parse the response body
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    
    // Check the response body
    assert_eq!(body, json!({"username": username}));
    
    // Try to create the same user again
    let response = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/create")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": username,
                        "password": password,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Check the response (should be an error)
    assert_eq!(response.status(), StatusCode::PAYMENT_REQUIRED);
}

#[tokio::test]
async fn test_auth_user() {
    // Create a test app
    let (app, _) = create_test_app().await;
    
    // Create a user
    let (username, password) = default_test_user();
    let _ = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/create")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": username,
                        "password": password,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Authenticate the user
    let response = app.clone()
        .oneshot(
            add_auth_headers(
                Request::builder()
                    .uri("/users/auth"),
                &username,
                &password
            )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Check the response
    assert_eq!(response.status(), StatusCode::OK);
    
    // Parse the response body
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    
    // Check the response body
    assert_eq!(body, json!({"authorized": "OK"}));
    
    // Try to authenticate with invalid credentials
    let response = app.clone()
        .oneshot(
            add_auth_headers(
                Request::builder()
                    .uri("/users/auth"),
                &username,
                "wrongpassword"
            )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Check the response (should be unauthorized)
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_progress_sync() {
    // Create a test app
    let (app, _) = create_test_app().await;
    
    // Create a user
    let (username, password) = default_test_user();
    let _ = app.clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users/create")
                .header("Content-Type", "application/json")
                .body(Body::from(
                    serde_json::to_string(&json!({
                        "username": username,
                        "password": password,
                    }))
                    .unwrap(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Create a progress state
    let progress_state = default_test_progress_state();
    let document = progress_state.document.clone();
    
    // Update progress
    let response = app.clone()
        .oneshot(
            add_auth_headers(
                Request::builder()
                    .method("PUT")
                    .uri("/syncs/progress")
                    .header("Content-Type", "application/json"),
                &username,
                &password
            )
                .body(Body::from(serde_json::to_string(&progress_state).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Check the response
    assert_eq!(response.status(), StatusCode::OK);
    
    // Get progress
    let response = app.clone()
        .oneshot(
            add_auth_headers(
                Request::builder()
                    .uri(&format!("/syncs/progress/{}", document)),
                &username,
                &password
            )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Check the response
    assert_eq!(response.status(), StatusCode::OK);
    
    // Parse the response body
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: ProgressState = serde_json::from_slice(&body).unwrap();
    
    // Check the response body
    assert_eq!(body.document, progress_state.document);
    assert_eq!(body.percentage, progress_state.percentage);
    assert_eq!(body.progress, progress_state.progress);
    assert_eq!(body.device, progress_state.device);
    assert_eq!(body.device_id, progress_state.device_id);
    // Timestamp will be updated by the server, so we don't check it
}