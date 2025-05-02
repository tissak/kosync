//! End-to-end tests for complete workflows

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
    fixtures::{test_progress_state, test_user},
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
async fn test_complete_user_workflow() {
    // Create a test app
    let (app, _) = create_test_app().await;
    
    // Step 1: Create a user
    let (username, password) = test_user("e2e_user", "e2e_password");
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
    
    // Step 2: Authenticate the user
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
    
    // Step 3: Update progress for a document
    let progress_state = test_progress_state(
        "e2e_document.epub",
        0.25,
        "25%",
        "e2e_device",
    );
    let document = progress_state.document.clone();
    
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
    
    // Step 4: Get progress for the document
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
    
    // Step 5: Update progress again with a new percentage
    let updated_progress_state = test_progress_state(
        "e2e_document.epub",
        0.5,
        "50%",
        "e2e_device",
    );
    
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
                .body(Body::from(serde_json::to_string(&updated_progress_state).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Check the response
    assert_eq!(response.status(), StatusCode::OK);
    
    // Step 6: Get progress again to verify the update
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
    assert_eq!(body.document, updated_progress_state.document);
    assert_eq!(body.percentage, updated_progress_state.percentage);
    assert_eq!(body.progress, updated_progress_state.progress);
    assert_eq!(body.device, updated_progress_state.device);
    assert_eq!(body.device_id, updated_progress_state.device_id);
    // Timestamp will be updated by the server, so we don't check it
}

#[tokio::test]
async fn test_multi_device_sync() {
    // Create a test app
    let (app, _) = create_test_app().await;
    
    // Step 1: Create a user
    let (username, password) = test_user("multi_device_user", "multi_device_password");
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
    
    // Step 2: Update progress from device 1
    let progress_state_device1 = test_progress_state(
        "multi_device_document.epub",
        0.25,
        "25%",
        "device1",
    );
    let document = progress_state_device1.document.clone();
    
    let _ = app.clone()
        .oneshot(
            add_auth_headers(
                Request::builder()
                    .method("PUT")
                    .uri("/syncs/progress")
                    .header("Content-Type", "application/json"),
                &username,
                &password
            )
                .body(Body::from(serde_json::to_string(&progress_state_device1).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Step 3: Get progress from device 2
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
    
    // Check the response body (device 2 should see device 1's progress)
    assert_eq!(body.document, progress_state_device1.document);
    assert_eq!(body.percentage, progress_state_device1.percentage);
    assert_eq!(body.progress, progress_state_device1.progress);
    assert_eq!(body.device, progress_state_device1.device);
    assert_eq!(body.device_id, progress_state_device1.device_id);
    
    // Step 4: Update progress from device 2
    let progress_state_device2 = test_progress_state(
        "multi_device_document.epub",
        0.5,
        "50%",
        "device2",
    );
    
    let _ = app.clone()
        .oneshot(
            add_auth_headers(
                Request::builder()
                    .method("PUT")
                    .uri("/syncs/progress")
                    .header("Content-Type", "application/json"),
                &username,
                &password
            )
                .body(Body::from(serde_json::to_string(&progress_state_device2).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    
    // Step 5: Get progress from device 1 again
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
    
    // Check the response body (device 1 should now see device 2's progress)
    assert_eq!(body.document, progress_state_device2.document);
    assert_eq!(body.percentage, progress_state_device2.percentage);
    assert_eq!(body.progress, progress_state_device2.progress);
    assert_eq!(body.device, progress_state_device2.device);
    assert_eq!(body.device_id, progress_state_device2.device_id);
}