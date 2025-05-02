# Product Context

This file provides a high-level overview of the project and the expected product that will be created. Initially it is based upon projectBrief.md (if provided) and all other available project-related information in the working directory. This file is intended to be updated as the project evolves, and should be used to inform all other modes of the project's goals and context.
2025-05-02 12:44:15 - Initial creation of the Memory Bank.

## Project Goal

KOSync is a progress synchronization server for KOReader, an e-book reader application. The server allows users to synchronize their reading progress across multiple devices. It's designed to be lightweight, efficient, and compatible with the existing koreader-sync-server API but uses sled instead of Redis for data storage.

## Key Features

- User authentication system
- Reading progress synchronization across devices
- Document-based progress tracking (percentage, position)
- RESTful API compatible with koreader-sync-server
- Lightweight and efficient storage using sled database
- Docker support for easy deployment
- Health check endpoint for monitoring

## Overall Architecture

The application is built using Rust with the following components:

1. **Web Framework**: Axum for handling HTTP requests and routing
2. **Database**: Sled for efficient key-value storage
3. **API Endpoints**:
   - `/users/create` - Create a new user
   - `/users/auth` - Authenticate a user
   - `/syncs/progress` - Update reading progress
   - `/syncs/progress/:doc` - Get reading progress for a specific document
   - `/healthcheck` - Check server health

The server uses a simple authentication mechanism with username and password stored in the sled database. Reading progress is stored as JSON objects with document ID, percentage, progress position, device information, and timestamp.