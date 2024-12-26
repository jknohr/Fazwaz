# Error Module

Centralized error handling for the system.

## Components

### Error Types
- `Error` - Custom error enum
  - Database errors
  - Storage errors
  - Validation errors
  - Not found errors

### Features
- Error context preservation
- Error conversion traits
- Integration with anyhow/thiserror
- Structured error responses 