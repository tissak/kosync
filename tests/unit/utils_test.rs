//! Unit tests for utility functions

use kosync::utils::{is_valid_field, is_valid_key_field, now_timestamp};

#[test]
fn test_is_valid_field() {
    // Valid fields
    assert!(is_valid_field("test"));
    assert!(is_valid_field("test123"));
    assert!(is_valid_field("test:123"));
    
    // Invalid fields
    assert!(!is_valid_field(""));
    
    // Create a string that exceeds the field length limit
    let long_string = "a".repeat(5000);
    assert!(!is_valid_field(&long_string));
}

#[test]
fn test_is_valid_key_field() {
    // Valid key fields
    assert!(is_valid_key_field("test"));
    assert!(is_valid_key_field("test123"));
    
    // Invalid key fields
    assert!(!is_valid_key_field(""));
    assert!(!is_valid_key_field("test:123")); // Contains colon
    
    // Create a string that exceeds the field length limit
    let long_string = "a".repeat(5000);
    assert!(!is_valid_key_field(&long_string));
}

#[test]
fn test_now_timestamp() {
    // Test that now_timestamp returns a non-zero value
    let timestamp = now_timestamp();
    assert!(timestamp > 0);
    
    // Test that now_timestamp increases over time
    std::thread::sleep(std::time::Duration::from_millis(10));
    let new_timestamp = now_timestamp();
    
    // Note: This test might fail if the system time resolution is too low
    // or if the system clock is adjusted during the test
    assert!(new_timestamp >= timestamp);
}

// Property-based tests using proptest
#[cfg(test)]
mod proptest_tests {
    use super::*;
    use proptest::prelude::*;
    
    proptest! {
        #[test]
        fn test_valid_fields_proptest(s in "[a-zA-Z0-9]{1,100}") {
            assert!(is_valid_field(&s));
        }
        
        #[test]
        fn test_valid_key_fields_proptest(s in "[a-zA-Z0-9]{1,100}") {
            assert!(is_valid_key_field(&s));
        }
        
        #[test]
        fn test_invalid_key_fields_proptest(s in "[a-zA-Z0-9]+:[a-zA-Z0-9]+") {
            assert!(!is_valid_key_field(&s));
        }
    }
}