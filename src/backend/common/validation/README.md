# Validation Module

Input validation logic for the Neural Reef Registration System.

## Components

### Image Validation (`image_validation.rs`)
Image file validation utilities:
- Size limits enforcement
- Format verification
- Content type validation
- Multipart form handling

### Listing Validation (`listing_validation.rs`)
Listing data validation:
- Required field validation
- Data format verification
- Business rule enforcement
- Cross-field validation

## Usage

```rust
use common::validation::{
    image_validation::validate_image_file,
    listing_validation::validate_listing_data
};

// Image validation
let validated_file = validate_image_file(&mut field).await?;

// Listing validation
let validated_listing = validate_listing_data(listing_request)?;
```

## Constants
- Maximum file sizes
- Allowed MIME types
- Field length limits
- Validation patterns 