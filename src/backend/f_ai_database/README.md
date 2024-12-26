# F AI Database Module

Core database functionality for the Neural Reef Registration System.

## Components

### Models
- `image_model.rs` - Handles image metadata storage and retrieval
- `listing_model.rs` - Manages property listing data and validation
- `batch_model.rs` - Handles batch operations

### Services
- `image_service.rs` - Image processing and storage service with B2 integration
- `listing_service.rs` - Property listing management with caching and async operations

### Configuration
- `config.rs` - Database configuration and connection settings
- `error.rs` - Custom error types for database operations

### Key Features
- Async/concurrent operations with proper error handling
- Integration with B2 storage
- Caching layer for performance
- Structured logging with tracing
- Transaction support
- Batch processing capabilities 