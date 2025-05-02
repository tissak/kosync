//! Unit tests for database operations

use kosync::db::DB;
use kosync::defs::ProgressState;
use std::path::PathBuf;
use tempfile::tempdir;

// Import test utilities
use crate::common::fixtures::{default_test_progress_state, default_test_user};

#[test]
fn test_db_creation() {
    // Create a temporary directory for the test database
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_db");
    
    // Create a new database
    let db = DB::new(&db_path);
    assert!(db.is_ok(), "Failed to create database");
}

#[test]
fn test_user_operations() {
    // Create a temporary directory for the test database
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_db");
    
    // Create a new database
    let db = DB::new(&db_path).expect("Failed to create database");
    
    // Get a user that doesn't exist
    let (username, password) = default_test_user();
    let result = db.get_user(&username);
    assert!(result.is_ok(), "Failed to get user");
    assert!(result.unwrap().is_none(), "User should not exist");
    
    // Put a user
    let put_result = db.put_user(&username, &password);
    assert!(put_result.is_ok(), "Failed to put user");
    
    // Get the user we just put
    let get_result = db.get_user(&username);
    assert!(get_result.is_ok(), "Failed to get user");
    let user_data = get_result.unwrap();
    assert!(user_data.is_some(), "User should exist");
    assert_eq!(
        user_data.unwrap().to_vec(),
        password.as_bytes(),
        "Password doesn't match"
    );
}

#[test]
fn test_doc_operations() {
    // Create a temporary directory for the test database
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let db_path = temp_dir.path().join("test_db");
    
    // Create a new database
    let db = DB::new(&db_path).expect("Failed to create database");
    
    // Create test data
    let (username, _) = default_test_user();
    let progress_state = default_test_progress_state();
    let document = &progress_state.document;
    
    // Get a document that doesn't exist
    let result = db.get_doc(&username, document);
    assert!(result.is_ok(), "Failed to get document");
    assert!(result.unwrap().is_none(), "Document should not exist");
    
    // Put a document
    let put_result = db.put_doc(&username, document, &progress_state);
    assert!(put_result.is_ok(), "Failed to put document");
    
    // Get the document we just put
    let get_result = db.get_doc(&username, document);
    assert!(get_result.is_ok(), "Failed to get document");
    let doc_data = get_result.unwrap();
    assert!(doc_data.is_some(), "Document should exist");
    
    // Compare the document data
    let retrieved_state = doc_data.unwrap();
    assert_eq!(retrieved_state.document, progress_state.document);
    assert_eq!(retrieved_state.percentage, progress_state.percentage);
    assert_eq!(retrieved_state.progress, progress_state.progress);
    assert_eq!(retrieved_state.device, progress_state.device);
    assert_eq!(retrieved_state.device_id, progress_state.device_id);
    assert_eq!(retrieved_state.timestamp, progress_state.timestamp);
}

// Test using the mock database
#[test]
fn test_mock_db() {
    use crate::common::mock_db::MockDB;
    
    // Create a mock database
    let db = MockDB::new();
    
    // Test user operations
    let (username, password) = default_test_user();
    let put_result = db.put_user(&username, &password);
    assert!(put_result.is_ok(), "Failed to put user in mock DB");
    
    let get_result = db.get_user(&username);
    assert!(get_result.is_ok(), "Failed to get user from mock DB");
    let user_data = get_result.unwrap();
    assert!(user_data.is_some(), "User should exist in mock DB");
    assert_eq!(
        user_data.unwrap().to_vec(),
        password.as_bytes(),
        "Password doesn't match in mock DB"
    );
    
    // Test document operations
    let progress_state = default_test_progress_state();
    let document = &progress_state.document;
    
    let put_doc_result = db.put_doc(&username, document, &progress_state);
    assert!(put_doc_result.is_ok(), "Failed to put document in mock DB");
    
    let get_doc_result = db.get_doc(&username, document);
    assert!(get_doc_result.is_ok(), "Failed to get document from mock DB");
    let doc_data = get_doc_result.unwrap();
    assert!(doc_data.is_some(), "Document should exist in mock DB");
    
    // Compare the document data
    let retrieved_state = doc_data.unwrap();
    assert_eq!(retrieved_state.document, progress_state.document);
    assert_eq!(retrieved_state.percentage, progress_state.percentage);
    assert_eq!(retrieved_state.progress, progress_state.progress);
    assert_eq!(retrieved_state.device, progress_state.device);
    assert_eq!(retrieved_state.device_id, progress_state.device_id);
    assert_eq!(retrieved_state.timestamp, progress_state.timestamp);
}