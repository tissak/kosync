//! Main test file for kosync

// Import common test utilities
mod common;

// Import unit tests
mod unit {
    mod utils_test;
    mod db_test;
}

// Import integration tests
mod integration {
    mod api_test;
}

// Import end-to-end tests
mod e2e {
    mod workflow_test;
}

// Import security tests
mod security {
    mod auth_test;
}

// Import performance tests
mod performance {
    mod benchmarks;
}

// This function is required to make the tests work with the common module
#[test]
fn it_works() {
    assert!(true);
}