# Transaction Storage Module

Handles external storage operations and file management.

## Components

### B2 Storage
- `b2_storage.rs` - Backblaze B2 cloud storage integration
  - File upload/download
  - Deletion operations
  - Metrics tracking
  - Error handling

### Features
- Async file operations
- Metrics collection
- Structured error handling
- UUID7-based file naming
- Content type handling 