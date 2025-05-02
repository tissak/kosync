# Test Infrastructure Implementation Plan

## Current Issues

We've encountered several issues while setting up the testing infrastructure:

1. **Auth Headers Function**: We changed the function signature from returning HeaderMap to modifying a Request::Builder, but we still have several places using the old function signature.

2. **Router Cloning**: The Router type doesn't implement Clone, so we need to clone it before each use.

3. **MockDB Duplication**: We have a naming conflict with MockDB.

## Implementation Steps

### 1. Fix Remaining Auth Headers Usage

The following files still have instances of the old `auth_headers` function that need to be updated to use `add_auth_headers`:

- `tests/e2e/workflow_test.rs` (lines 301 and 314)
- `tests/security/auth_test.rs` (multiple instances)

### 2. Fix App Cloning Issues

In several test files, we're trying to use the `app` variable after it has been moved. We need to clone the Router before each use:

```rust
// Before
let response = app.oneshot(...);

// After
let response = app.clone().oneshot(...);
```

### 3. Fix MockDB Duplication

In `tests/common/mock_db.rs`, we have a duplicate definition of MockDB. We need to:

1. Keep only one implementation of MockDB
2. Use a trait to define the interface
3. Use mockall to create a mock implementation of the trait

### 4. Run Specific Tests

Once the fixes are in place, we should run specific tests to verify our changes:

```bash
cargo test test_is_valid_field
```

## Next Steps

After completing the test infrastructure setup, we should:

1. Implement unit tests for utility functions
2. Create tests for database operations
3. Develop tests for API endpoints
4. Test authentication middleware

## Conclusion

The testing infrastructure setup is a critical first step in ensuring the reliability and quality of the KOSync application. By addressing these issues and implementing a comprehensive testing strategy, we can build a robust foundation for future development.