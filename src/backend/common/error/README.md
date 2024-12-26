# Error Module

Centralized error handling for the Neural Reef Registration System.

## Components

### Error Types (`error_types.rs`)
Custom error types for different domains:
- `ImageError` - Image processing errors
- `ValidationError` - Input validation errors
- `DatabaseError` - Database operation errors
- `AuthError` - Authentication/authorization errors

### Result Type
```rust
pub type Result<T> = std::result::Result<T, Error>;
```

## Features
- Error context preservation
- Structured error responses
- Error conversion traits
- Logging integration
- HTTP status code mapping 