-- Define tables with SCHEMAFULL
DEFINE TABLE listing SCHEMAFULL;
DEFINE FIELD id ON listing TYPE record<listing>;
DEFINE FIELD title ON listing TYPE string;
DEFINE FIELD description ON listing TYPE string;
DEFINE FIELD property_type ON listing TYPE string;
DEFINE FIELD ownership_type ON listing TYPE string;
DEFINE FIELD land_ownership_type ON listing TYPE string;
DEFINE FIELD asking_price ON listing TYPE float;
DEFINE FIELD long_term_rental ON listing TYPE float;
DEFINE FIELD short_term_rental ON listing TYPE float;
DEFINE FIELD bedrooms ON listing TYPE int;
DEFINE FIELD bathrooms ON listing TYPE int;
DEFINE FIELD indoor_square_meters ON listing TYPE int;
DEFINE FIELD outdoor_square_meters ON listing TYPE int;
DEFINE FIELD plot_size ON listing TYPE int;
DEFINE FIELD unit_number ON listing TYPE string;
DEFINE FIELD storeys ON listing TYPE string;
DEFINE FIELD year_of_completion ON listing TYPE string;
DEFINE FIELD amenities ON listing TYPE array;
DEFINE FIELD available_from ON listing TYPE datetime;
DEFINE FIELD available_to ON listing TYPE datetime;
DEFINE FIELD available_as ON listing TYPE string;
DEFINE FIELD created_at ON listing TYPE datetime;
DEFINE FIELD updated_at ON listing TYPE datetime;
DEFINE FIELD status ON listing TYPE string;
DEFINE FIELD country ON listing TYPE string;
DEFINE FIELD district ON listing TYPE string;
DEFINE FIELD subdistrict ON listing TYPE string;

-- Define image table
DEFINE TABLE image SCHEMAFULL;
DEFINE FIELD id ON image TYPE record<image>;
DEFINE FIELD listing_id ON image TYPE record<listing>;
DEFINE FIELD filename ON image TYPE string;
DEFINE FIELD content_type ON image TYPE string;
DEFINE FIELD size ON image TYPE int;
DEFINE FIELD width ON image TYPE int;
DEFINE FIELD height ON image TYPE int;
DEFINE FIELD created_at ON image TYPE datetime;
DEFINE FIELD updated_at ON image TYPE datetime;

-- Define indexes
DEFINE INDEX listing_country ON listing FIELDS country;
DEFINE INDEX listing_district ON listing FIELDS district;
DEFINE INDEX listing_status ON listing FIELDS status;
DEFINE INDEX image_listing ON image FIELDS listing_id; 