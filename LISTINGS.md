# Neural Reef System Architecture

## Core Files Structure

### Root Level Files
```
src/
├── config.rs         # Central configuration management
├── error.rs         # Error types and handling
└── main.rs          # Application entry point and service initialization
```

## Directory Responsibilities

### Config (`src/config/`)
```
config/
└── logging.rs       # Logging configuration and initialization
```
- Configures tracing subscriber
- Sets up logging filters and formatting
- Initializes logging system at startup

### Database (`src/db/`)
```
db/
├── mod.rs          # Database connection and operations
└── schema.sql      # SurrealDB schema definitions
```
- Manages SurrealDB connections
- Implements database operations
- Defines table schemas and relationships

### Handlers (`src/handlers/`)
```
handlers/
├── image.rs        # Image upload and processing endpoints
├── key.rs          # API key management endpoints
├── key_test.rs     # Tests for key management
├── mod.rs          # Handler registration
└── search.rs       # Image search endpoints
```
- HTTP request handling
- Input validation
- Response formatting
- Route-specific logic

### Jobs (`src/jobs/`)
```
jobs/
├── image_analysis.rs  # Background image analysis tasks
└── key_cleanup.rs     # API key expiration cleanup
```
- Background task processing
- Scheduled maintenance
- Asynchronous operations

### Middleware (`src/middleware/`)
```
middleware/
├── auth.rs          # Authentication middleware
└── rate_limit.rs    # Rate limiting implementation
```
- Request authentication
- Rate limiting
- Request/Response processing

### Models (`src/models/`)
```
models/
├── batch.rs         # Batch processing data structures
└── listing.rs       # Real estate listing models
```
- Data structure definitions
- Serialization/Deserialization
- Business logic models

### Monitoring (`src/monitoring/`)
```
monitoring/
└── metrics.rs       # System metrics collection
```
- Performance metrics
- System monitoring
- Statistics collection

### Services (`src/services/`)
```
services/
├── storage/
│   ├── b2.rs        # Backblaze B2 storage integration
│   └── temp.rs      # Temporary file storage
├── batch_processor.rs # Batch operation handling
├── cache.rs         # Caching implementation
├── email.rs         # Email service
├── embedding.rs     # Vector embedding generation
├── image.rs         # Image processing
├── image_test.rs    # Image service tests
├── key.rs          # API key operations
├── key_service.rs   # Key management service
├── listing_service.rs # Listing management
├── prompt.rs        # AI prompt management
└── resource.rs      # Resource management
```
- Core business logic
- External service integration
- Data processing
- Resource management

### Templates (`src/templates/`)
```
templates/
├── key_email.html    # HTML email template
└── key_email_text.txt # Plain text email template
```
- Email templates
- Notification formatting

### Assets (`src/assets/`)
```
assets/
└── watermark.svg     # Image watermark template
```
- Static assets
- Resource files

## Key Components Interaction

1. **Request Flow**
   ```
   Client -> Middleware -> Handler -> Service -> Database/Storage
   ```

2. **Image Processing Flow**
   ```
   Upload -> Validation -> Processing -> Storage -> Analysis -> Database
   ```

3. **Background Jobs**
   ```
   Scheduler -> Job -> Service -> Database/Storage
   ```

## Service Dependencies

1. **Image Service**
   - Storage Service
   - Embedding Service
   - Cache Service
   - Resource Manager

2. **Listing Service**
   - Database
   - Image Service

3. **Key Service**
   - Database
   - Email Service

## Error Handling

Error propagation follows this hierarchy:
```
Service Error -> Handler Error -> HTTP Response
```

## Metrics Collection

Metrics are collected at multiple levels:
- Request/Response timing
- Processing durations
- Resource usage
- Error rates
- Cache performance

## Configuration Management

Configuration is loaded in this order:
1. Default values
2. Configuration files
3. Environment variables
4. Runtime overrides

## Configuration System (`src/config.rs`)

### Core Configuration Structure
```rust
pub struct Settings {
    pub server: ServerConfig,      // Server settings
    pub storage: StorageConfig,    // B2 storage configuration
    pub surreal: SurrealConfig,    // Database settings
    pub email: EmailConfig,        // Email service settings
    pub api: ApiConfig,            // API configuration
    pub key: KeyConfig,            // Key management settings
    pub cache: CacheConfig,        // Cache settings
    pub resource: ResourceConfig,  // Resource limits
    pub maintenance: MaintenanceConfig, // Maintenance tasks
    pub openai: OpenAIConfig,      // OpenAI integration
}
```

### Component Configurations

#### Server Settings
```rust
pub struct ServerConfig {
    pub host: String,    // Server host address
    pub port: u16,       // Server port number
}
```

#### Storage Settings
```rust
pub struct StorageConfig {
    pub b2_key_id: String,      // Backblaze B2 key ID
    pub b2_key: String,         // Backblaze B2 application key
    pub b2_bucket_id: String,   // Target bucket identifier
}
```

#### Resource Management
```rust
pub struct ResourceConfig {
    pub max_concurrent_uploads: usize,    // Max parallel uploads
    pub max_concurrent_processing: usize, // Max processing jobs
    pub max_concurrent_searches: usize,   // Max parallel searches
    pub max_file_size: usize,            // Max file size limit
    pub allowed_mime_types: Vec<String>, // Allowed file types
}
```

#### Maintenance Configuration
```rust
pub struct MaintenanceConfig {
    pub cache_cleanup_interval: u64,     // Cache cleanup frequency
    pub embedding_update_interval: u64,  // Embedding update timing
    pub key_cleanup_interval: u64,      // Key expiration checks
    pub max_stale_embedding_age: u64,   // Max embedding age
}
```

### Configuration Loading

The system loads configuration in this order:

1. **Default Configuration**
```rust
s.merge(File::with_name("config/default"))?;
```

2. **Environment Overrides**
```rust
let env = std::env::var("RUN_ENV")
    .unwrap_or_else(|_| "development".into());
s.merge(File::with_name(&format!("config/{}", env)))?;
```

3. **Environment Variables**
```rust
s.merge(Environment::with_prefix("app"))?;
```

### Environment Variables Example
```env
APP_SERVER_HOST=0.0.0.0
APP_SERVER_PORT=3000
APP_STORAGE_B2_KEY_ID=your_key_id
APP_SURREAL_URL=http://localhost:8000
APP_OPENAI_API_KEY=your_openai_key
```

## Testing Structure

- Unit tests alongside implementation files
- Integration tests in `tests/` directory
- Mock implementations for external services
- Test utilities and helpers

Would you like me to expand on any particular component or add more details about specific interactions?