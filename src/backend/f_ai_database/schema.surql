-- Node Tables (Vertices)
DEFINE TABLE listing SCHEMAFULL;
DEFINE FIELD listing_id ON listing TYPE string ASSERT $value != NONE;
DEFINE FIELD title ON listing TYPE string;
DEFINE FIELD description ON listing TYPE string;
DEFINE FIELD property_type ON listing TYPE string;
DEFINE FIELD status ON listing TYPE string ASSERT $value != NONE;
DEFINE FIELD created_at ON listing TYPE datetime ASSERT $value != NONE;
DEFINE FIELD updated_at ON listing TYPE datetime ASSERT $value != NONE;

DEFINE TABLE property_details SCHEMAFULL;
DEFINE FIELD property_type ON property_details TYPE string;
DEFINE FIELD furnishing ON property_details TYPE string;
DEFINE FIELD condition ON property_details TYPE string;
DEFINE FIELD view_type ON property_details TYPE array;
DEFINE FIELD parking ON property_details TYPE object;
DEFINE FIELD created_at ON property_details TYPE datetime;

DEFINE TABLE price SCHEMAFULL;
DEFINE FIELD amount ON price TYPE float;
DEFINE FIELD currency ON price TYPE string;
DEFINE FIELD price_type ON price TYPE string;
DEFINE FIELD created_at ON price TYPE datetime;

DEFINE TABLE dimension SCHEMAFULL;
DEFINE FIELD bedrooms ON dimension TYPE int;
DEFINE FIELD bathrooms ON dimension TYPE int;
DEFINE FIELD indoor_area ON dimension TYPE int;
DEFINE FIELD outdoor_area ON dimension TYPE int;
DEFINE FIELD plot_size ON dimension TYPE int;
DEFINE FIELD created_at ON dimension TYPE datetime;

DEFINE TABLE batch SCHEMAFULL;
DEFINE FIELD batch_id ON batch TYPE string ASSERT $value != NONE;
DEFINE FIELD status ON batch TYPE string;
DEFINE FIELD total_images ON batch TYPE int;
DEFINE FIELD processed_images ON batch TYPE int;
DEFINE FIELD failed_images ON batch TYPE int;
DEFINE FIELD created_at ON batch TYPE datetime;
DEFINE FIELD updated_at ON batch TYPE datetime;

DEFINE TABLE image SCHEMAFULL;
DEFINE FIELD image_id ON image TYPE string ASSERT $value != NONE;
DEFINE FIELD filename ON image TYPE string;
DEFINE FIELD status ON image TYPE string;
DEFINE FIELD analysis_result ON image TYPE object;
DEFINE FIELD created_at ON image TYPE datetime;
DEFINE FIELD updated_at ON image TYPE datetime;

DEFINE TABLE location SCHEMAFULL;
DEFINE FIELD country ON location TYPE string;
DEFINE FIELD province ON location TYPE string;
DEFINE FIELD district ON location TYPE string;
DEFINE FIELD coordinates ON location TYPE object;
DEFINE FIELD created_at ON location TYPE datetime;

-- Edge Tables (Relationships)
DEFINE TABLE owns_listing SCHEMAFULL;
DEFINE FIELD in ON owns_listing TYPE record<user>;
DEFINE FIELD out ON owns_listing TYPE record<listing>;
DEFINE FIELD ownership_type ON owns_listing TYPE string;
DEFINE FIELD created_at ON owns_listing TYPE datetime;

DEFINE TABLE has_details SCHEMAFULL;
DEFINE FIELD in ON has_details TYPE record<listing>;
DEFINE FIELD out ON has_details TYPE record<property_details>;
DEFINE FIELD created_at ON has_details TYPE datetime;

DEFINE TABLE has_batch SCHEMAFULL;
DEFINE FIELD in ON has_batch TYPE record<listing>;
DEFINE FIELD out ON has_batch TYPE record<batch>;
DEFINE FIELD created_at ON has_batch TYPE datetime;

DEFINE TABLE contains_image SCHEMAFULL;
DEFINE FIELD in ON contains_image TYPE record<batch>;
DEFINE FIELD out ON contains_image TYPE record<image>;
DEFINE FIELD created_at ON contains_image TYPE datetime;

DEFINE TABLE has_location SCHEMAFULL;
DEFINE FIELD in ON has_location TYPE record<listing>;
DEFINE FIELD out ON has_location TYPE record<location>;
DEFINE FIELD created_at ON has_location TYPE datetime;

DEFINE TABLE has_api_key SCHEMAFULL;
DEFINE FIELD in ON has_api_key TYPE record<listing>;
DEFINE FIELD out ON has_api_key TYPE record<api_key>;
DEFINE FIELD created_at ON has_api_key TYPE datetime;
DEFINE FIELD expires_at ON has_api_key TYPE datetime;
DEFINE FIELD revoked ON has_api_key TYPE bool DEFAULT false;

DEFINE TABLE has_price SCHEMAFULL;
DEFINE FIELD in ON has_price TYPE record<listing>;
DEFINE FIELD out ON has_price TYPE record<price>;
DEFINE FIELD price_type ON has_price TYPE string;
DEFINE FIELD created_at ON has_price TYPE datetime;

DEFINE TABLE has_dimension SCHEMAFULL;
DEFINE FIELD in ON has_dimension TYPE record<listing>;
DEFINE FIELD out ON has_dimension TYPE record<dimension>;
DEFINE FIELD created_at ON has_dimension TYPE datetime;

-- Image relationships
DEFINE TABLE has_image SCHEMAFULL;
DEFINE FIELD in ON has_image TYPE record<listing>;
DEFINE FIELD out ON has_image TYPE record<image>;
DEFINE FIELD is_primary ON has_image TYPE bool DEFAULT false;
DEFINE FIELD batch_id ON has_image TYPE string;
DEFINE FIELD created_at ON has_image TYPE datetime;

-- Indexes
DEFINE INDEX listing_id_idx ON listing FIELDS listing_id UNIQUE;
DEFINE INDEX batch_id_idx ON batch FIELDS batch_id UNIQUE;
DEFINE INDEX image_id_idx ON image FIELDS image_id UNIQUE;
DEFINE INDEX location_coords_idx ON location FIELDS coordinates;

-- Graph Relationship Indexes
DEFINE INDEX owns_listing_idx ON owns_listing COLUMNS in, out UNIQUE;
DEFINE INDEX has_details_idx ON has_details COLUMNS in, out UNIQUE;
DEFINE INDEX has_batch_idx ON has_batch COLUMNS in, out UNIQUE;
DEFINE INDEX contains_image_idx ON contains_image COLUMNS in, out UNIQUE;
DEFINE INDEX has_location_idx ON has_location COLUMNS in, out UNIQUE;
DEFINE INDEX has_api_key_idx ON has_api_key COLUMNS in, out UNIQUE;
DEFINE INDEX has_price_idx ON has_price COLUMNS in, out;
DEFINE INDEX has_dimension_idx ON has_dimension COLUMNS in, out UNIQUE;

-- Add index for image relationships
DEFINE INDEX has_image_idx ON has_image COLUMNS in, out UNIQUE;
DEFINE INDEX batch_image_idx ON has_image COLUMNS batch_id;

-- Permissions and Scopes
DEFINE SCOPE listing_scope SESSION 24h
    SIGNUP ( CREATE listing SET 
        listing_id = $listing_id,
        title = $title,
        created_at = time::now(),
        updated_at = time::now()
    )
    SIGNIN ( SELECT * FROM listing WHERE listing_id = $listing_id ); 

-- Additional Edge Tables needed
DEFINE TABLE has_amenity SCHEMAFULL;
DEFINE FIELD in ON has_amenity TYPE record<listing>;
DEFINE FIELD out ON has_amenity TYPE record<amenity>;
DEFINE FIELD created_at ON has_amenity TYPE datetime;

DEFINE TABLE has_view SCHEMAFULL;
DEFINE FIELD in ON has_view TYPE record<property_details>;
DEFINE FIELD out ON has_view TYPE record<view>;
DEFINE FIELD created_at ON has_view TYPE datetime;

-- Additional Node Tables needed
DEFINE TABLE amenity SCHEMAFULL;
DEFINE FIELD name ON amenity TYPE string;
DEFINE FIELD category ON amenity TYPE string;
DEFINE FIELD created_at ON amenity TYPE datetime;

DEFINE TABLE view SCHEMAFULL;
DEFINE FIELD type ON view TYPE string;
DEFINE FIELD description ON view TYPE string;
DEFINE FIELD created_at ON view TYPE datetime;

-- Additional Indexes
DEFINE INDEX has_amenity_idx ON has_amenity COLUMNS in, out;
DEFINE INDEX has_view_idx ON has_view COLUMNS in, out; 