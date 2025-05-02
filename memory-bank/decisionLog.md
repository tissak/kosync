# Decision Log

This file records architectural and implementation decisions using a list format.
2025-05-02 12:45:11 - Initial creation of the Memory Bank.
2025-05-02 12:55:57 - Added testing strategy decision.
2025-05-02 13:30:00 - Added test infrastructure implementation decisions.
2025-05-02 14:39:00 - Added Docker Compose deployment for Raspberry Pi 4 decision.

## Decision

1. **Use of Sled Database**: The project uses sled instead of Redis (which is used by the original koreader-sync-server).

2. **Comprehensive Testing Strategy**: Implement a multi-layered testing approach including unit, integration, end-to-end, performance, and security testing.

3. **Test Infrastructure Design**: Implement a modular test infrastructure with shared utilities and fixtures.

4. **Authentication Helper Design**: Change the auth_headers function to modify a Request::Builder directly instead of returning HeaderMap.

5. **MockDB Implementation**: Use a trait-based approach with mockall for database mocking.

6. **Docker Compose Deployment for Raspberry Pi 4**: Create a Docker Compose configuration for easy deployment to Raspberry Pi 4 (ARM64 architecture).

## Rationale

1. **Sled vs Redis**: 
   - Sled is a pure Rust embedded database, which aligns with the project's Rust implementation.
   - It eliminates the need for a separate Redis server, simplifying deployment.
   - Sled is designed to be lightweight and efficient, which matches the project's goals.
   - The embedded nature of sled makes it easier to package the entire application, including its database, in a single binary or Docker container.

2. **Comprehensive Testing Strategy**:
   - Modern Rust testing practices emphasize a multi-layered approach to ensure robustness.
   - Unit tests provide quick feedback on individual components.
   - Integration tests verify that components work together correctly.
   - End-to-end tests validate complete workflows from a user perspective.
   - Performance testing ensures the application meets efficiency requirements.
   - Security testing identifies potential vulnerabilities.
   - This approach aligns with industry best practices and ensures high-quality, reliable code.

3. **Test Infrastructure Design**:
   - A modular approach allows for code reuse across different test types.
   - Shared utilities and fixtures reduce duplication and maintenance overhead.
   - Common test patterns can be standardized and improved over time.
   - This approach makes tests more maintainable and easier to write.

4. **Authentication Helper Design**:
   - The original design returned a HeaderMap, which required a separate step to add to the request.
   - The new design modifies the Request::Builder directly, which is more ergonomic and less error-prone.
   - This approach better aligns with the builder pattern used by the HTTP library.

5. **MockDB Implementation**:
   - Using a trait-based approach with mockall provides more flexibility for testing.
   - It allows for both a simple in-memory implementation and more complex mocking scenarios.
   - This approach better separates the interface from the implementation.
   - It makes tests more maintainable and easier to write.

6. **Docker Compose Deployment for Raspberry Pi 4**:
   - Docker Compose simplifies deployment by defining all services in a single YAML file.
   - ARM64-compatible images are required for Raspberry Pi 4 deployment.
   - Volume mapping ensures data persistence across container restarts.
   - Environment variables provide flexible configuration.
   - Restart policies ensure the service recovers from failures.
   - This approach makes deployment and maintenance easier for Raspberry Pi users.

## Implementation Details

1. **Sled Configuration**:
   - The database is configured with `LowSpace` mode to optimize for storage efficiency.
   - Cache capacity is set to 256KB.
   - Data is stored in a tree named "kosync".
   - Database files are stored in "data/kosync" by default.
   - Key Structure:
     - User keys follow the pattern `U:{username}:K`
     - Document keys follow the pattern `U:{username}:D:{document}`
     - This structure allows for efficient retrieval of user-specific data.

2. **Testing Implementation**:
   - Tests are organized in a structured directory hierarchy under `/tests`:
     - `/tests/unit/` - For unit tests of individual functions
     - `/tests/integration/` - For testing API endpoints and component interactions
     - `/tests/e2e/` - For end-to-end testing scenarios
     - `/tests/performance/` - For benchmarks and performance tests
     - `/tests/security/` - For security-related tests
     - `/tests/common/` - For shared test utilities and fixtures
   - Unit tests focus on individual functions and components.
   - Integration tests verify API endpoints and middleware.
   - End-to-end tests simulate real-world usage scenarios.
   - Performance tests measure response times and resource usage.
   - Security tests identify potential vulnerabilities.
   - Continuous integration will ensure tests are run automatically on each commit.
   - Code coverage metrics will be tracked to ensure comprehensive test coverage.

3. **Test Utilities Implementation**:
   - Authentication helpers in `tests/common/auth_helpers.rs` provide functions to add authentication headers to requests.
   - Fixtures in `tests/common/fixtures.rs` provide common test data.
   - Mock database in `tests/common/mock_db.rs` provides an in-memory implementation for testing.
   - These utilities are exposed through `tests/common/mod.rs`.

4. **Testing Dependencies**:
   - Proptest for property-based testing
   - Tower and Hyper for testing Axum applications
   - Reqwest for HTTP client testing
   - Mockall for mocking
   - Criterion for performance benchmarking
   - Tokio-test for async testing
   - Tempfile for temporary file creation

5. **Docker Compose Implementation for Raspberry Pi 4**:
   - Create a docker-compose.yml file with:
     - Service definition for KOSync
     - ARM64-compatible image specification
     - Volume mapping for data persistence
     - Port mapping for the server
     - Environment variable configuration
     - Restart policy for reliability
   - Update the Dockerfile to support ARM64 architecture
   - Update the build script to build for ARM64 architecture
   - Document the deployment process