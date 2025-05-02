//! Test fixtures for common test data

use kosync::defs::ProgressState;

/// Create a test user with the given name and password
pub fn test_user(name: &str, password: &str) -> (String, String) {
    (name.to_string(), password.to_string())
}

/// Create a default test user
pub fn default_test_user() -> (String, String) {
    test_user("testuser", "testpassword")
}

/// Create a test progress state for a document
pub fn test_progress_state(
    document: &str,
    percentage: f32,
    progress: &str,
    device: &str,
) -> ProgressState {
    ProgressState {
        document: document.to_string(),
        percentage,
        progress: progress.to_string(),
        device: device.to_string(),
        device_id: Some("test-device-id".to_string()),
        timestamp: Some(1234567890),
    }
}

/// Create a default test progress state
pub fn default_test_progress_state() -> ProgressState {
    test_progress_state(
        "test-document.epub",
        0.5,
        "50%",
        "test-device",
    )
}

/// Create multiple test users
pub fn multiple_test_users(count: usize) -> Vec<(String, String)> {
    (0..count)
        .map(|i| test_user(&format!("testuser{}", i), &format!("password{}", i)))
        .collect()
}

/// Create multiple test progress states
pub fn multiple_test_progress_states(count: usize) -> Vec<ProgressState> {
    (0..count)
        .map(|i| {
            test_progress_state(
                &format!("document{}.epub", i),
                i as f32 / count as f32,
                &format!("{}%", (i * 100) / count),
                &format!("device{}", i),
            )
        })
        .collect()
}