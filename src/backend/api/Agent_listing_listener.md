http
POST /agent/listing
{
"fullname": String, // Must include first and last name
"email": String, // Valid email format
"phone_number": String // International format
"country": String     // Must be one of our supported countries
}


## 2. Flow Sequence

### A. Listing Creation
1. Input Validation (`request_validation.rs`)
   - Email format (EMAIL_REGEX)
   - Phone format (PHONE_REGEX)
   - Name validation (two parts)

2. State Management (`state.rs`)
   - Coordinates services
   - Manages transactions
   - Handles rollbacks

3. Database Operations (`listing_model.rs`)
   - Creates listing record
   - Generates ListingId (UUID7)
   - Stores initial metadata

4. Key Management (`key_logic_auth/key_service.rs`)
   - Generates API key
   - Associates with listing
   - Sets expiry and permissions

5. Email Notification (`email/service.rs`)
   - Sends confirmation
   - Includes API key and instructions

### B. Image Processing Pipeline

1. Upload Handling (`api/image.rs`)
   - Validates API key
   - Checks file types/sizes
   - Creates batch record

2. Image Processing (`image_processor/`)
   - Resizing/optimization
   - Quality checks
   - Metadata extraction

3. Analysis Pipeline (`analysis_pipeline.rs`)
   - Feature extraction
   - Classification
   - Quality scoring

4. LLM Analysis (`llm_caller/`)
   - Description generation
   - Feature validation
   - Quality assessment

### C. Location Management

1. Location Service (`location_schema.rs`)
   - Mapbox integration
   - Geocoding
   - Address validation

2. Graph Relationships
   - Links listings to locations
   - Enables spatial queries
   - Maintains hierarchy

## 3. Database Schema


json
{
"listing_id": String,
"api_key": String
}

## 4. Database Schema
sql
-- Core listing record
DEFINE TABLE listings SCHEMAFULL;
DEFINE FIELD id ON listings TYPE string;
DEFINE FIELD listing_id ON listings TYPE string;
DEFINE FIELD api_key ON listings TYPE string;
DEFINE FIELD email ON listings TYPE string;
DEFINE FIELD fullname ON listings TYPE string;
DEFINE FIELD phone ON listings TYPE string;
DEFINE FIELD status ON listings TYPE string;
DEFINE FIELD created_at ON listings TYPE datetime;
DEFINE FIELD updated_at ON listings TYPE datetime;
-- API key relationship
DEFINE TABLE api_keys SCHEMAFULL;
DEFINE FIELD key ON api_keys TYPE string;
DEFINE FIELD listing_id ON api_keys TYPE string;

## 5. Error Handling
- Input validation errors (400)
- Database errors (500)
- Email service errors (500)
- Duplicate requests (409)

http
POST /api/agent/listing
{
"fullname": String, // Must include first and last name
"email": String, // Valid email format
"phone_number": String // International format
}
