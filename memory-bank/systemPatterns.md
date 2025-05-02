# System Patterns

This file documents recurring patterns and standards used in the project.
It is optional, but recommended to be updated as the project evolves.
2025-05-02 12:44:55 - Initial creation of the Memory Bank.
2025-05-02 12:55:30 - Added testing patterns based on the test plan.

## Coding Patterns

1. **Module Organization**: The project is organized into several modules:
   - `api.rs`: API endpoint handlers
   - `db.rs`: Database operations
   - `defs.rs`: Type definitions and constants
   - `utils.rs`: Utility functions
   - `main.rs`: Application entry point and server setup

2. **Error Handling**: The project uses a custom `Error` enum with a macro (`def_error!`) to define error types and their HTTP responses.

3. **Authentication Middleware**: Authentication is implemented as Axum middleware that checks for `x-auth-user` and `x-auth-key` headers.

4. **Database Key Structure**: 
   - User keys: `U:{username}:K`
   - Document keys: `U:{username}:D:{document}`

5. **Logging**: Uses tracing with different configurations for debug and release builds.

## Architectural Patterns

1. **State Management**: The application uses Axum's state management to share the database connection across handlers.

2. **Middleware Layers**: Authentication is implemented as middleware that can be applied to specific routes.

3. **Graceful Shutdown**: The server implements graceful shutdown on CTRL+C signal.

4. **Key-Value Storage**: Uses sled for efficient key-value storage with a tree-based structure.

## Testing Patterns

Based on the test plan, the following testing patterns will be implemented:

1. **Test Organization**: Tests will be organized in a structured hierarchy:
   ```
   /tests
     /unit
       /db
       /utils
       /api
     /integration
       /endpoints
       /middleware
     /e2e
       /workflows
     /performance
       /benchmarks
     /security
   ```

2. **Test-Driven Development**: New features should be implemented using TDD principles.

3. **Mocking**: External dependencies should be mocked for unit and integration tests.

4. **Parameterized Testing**: Tests should use parameterized inputs to cover edge cases.

5. **Continuous Integration**: Tests should be run automatically on each commit.

6. **Code Coverage**: Aim for high test coverage (>80%).

7. **Performance Benchmarking**: Use Criterion for performance benchmarking.

8. **Security Testing**: Include security-focused tests and static analysis.