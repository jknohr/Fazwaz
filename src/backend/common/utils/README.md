# Utils Module

Common utilities and helper functions for the Neural Reef Registration System.

## Components

### Pagination (`pagination.rs`)
Pagination utilities:
- Page size validation
- Offset calculation
- Cursor-based pagination
- Response formatting

### Rate Limiting (`rate_limiting.rs`)
Rate limiting utilities:
- Token bucket implementation
- Window-based limiting
- Rate limit configuration
- Limit enforcement

## Usage

```rust
use common::utils::{
    pagination::paginate_results,
    rate_limiting::check_rate_limit
};

// Pagination
let page = paginate_results(results, page_size, offset)?;

// Rate limiting
let allowed = check_rate_limit(api_key, limit_config).await?;
``` 