//! Helper functions for authentication in tests

use axum::http::request::Builder;

/// Add authentication headers to a request builder
pub fn add_auth_headers(builder: Builder, username: &str, password: &str) -> Builder {
    builder
        .header("x-auth-user", username)
        .header("x-auth-key", password)
}

/// Create a default set of authentication headers for a request builder
pub fn default_auth_headers(builder: Builder) -> Builder {
    add_auth_headers(builder, "testuser", "testpassword")
}

/// Create an authentication token (for future use if token-based auth is implemented)
pub fn create_auth_token(username: &str, password: &str) -> String {
    // This is a placeholder for future token-based authentication
    // In a real implementation, this would generate a JWT or other token
    format!("{}:{}", username, password)
}

/// Parse an authentication token (for future use if token-based auth is implemented)
pub fn parse_auth_token(token: &str) -> Option<(String, String)> {
    // This is a placeholder for future token-based authentication
    // In a real implementation, this would validate and parse a JWT or other token
    let parts: Vec<&str> = token.split(':').collect();
    if parts.len() == 2 {
        Some((parts[0].to_string(), parts[1].to_string()))
    } else {
        None
    }
}