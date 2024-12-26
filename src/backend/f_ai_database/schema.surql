-- Core tables
DEFINE TABLE images SCHEMAFULL;
DEFINE TABLE listings SCHEMAFULL;
DEFINE TABLE batches SCHEMAFULL;
DEFINE TABLE keys SCHEMAFULL;
DEFINE TABLE analysis_jobs SCHEMAFULL;

-- Image schema
DEFINE FIELD id ON images TYPE string;
DEFINE FIELD listing_id ON images TYPE string;
DEFINE FIELD filename ON images TYPE string;
DEFINE FIELD original_filename ON images TYPE string;
DEFINE FIELD b2_file_id ON images TYPE string;
DEFINE FIELD b2_bucket_id ON images TYPE string;
DEFINE FIELD b2_url ON images TYPE string;
DEFINE FIELD size ON images TYPE int;
DEFINE FIELD mime_type ON images TYPE string;
DEFINE FIELD width ON images TYPE int;
DEFINE FIELD height ON images TYPE int;
DEFINE FIELD created_at ON images TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON images TYPE datetime DEFAULT time::now();
DEFINE FIELD processed ON images TYPE bool DEFAULT false;
DEFINE FIELD watermarked ON images TYPE bool DEFAULT false;

-- Metadata and Analysis
DEFINE FIELD metadata ON images TYPE object {
    tags: array,
    objects: array,
    scene_type: string,
    color_palette: array,
    text_features: object {
        keywords: array,
        sentiment: string,
        property_features: object {
            bedrooms: number,
            bathrooms: number,
            square_meter: number,
            amenities: array
        }
    }
};

-- Text content for embedding
DEFINE FIELD text_content ON images TYPE string;

-- Analysis Jobs
DEFINE FIELD job_id ON analysis_jobs TYPE string;
DEFINE FIELD listing_id ON analysis_jobs TYPE string;
DEFINE FIELD image_id ON analysis_jobs TYPE string;
DEFINE FIELD b2_file_id ON analysis_jobs TYPE string;
DEFINE FIELD created_at ON analysis_jobs TYPE datetime;
DEFINE FIELD status ON analysis_jobs TYPE string;
DEFINE FIELD attempts ON analysis_jobs TYPE int;

-- Listing schema
DEFINE FIELD id ON listings TYPE string;
DEFINE FIELD title ON listings TYPE string;
DEFINE FIELD description ON listings TYPE string;
DEFINE FIELD status ON listings TYPE string;
DEFINE FIELD created_at ON listings TYPE datetime;
DEFINE FIELD updated_at ON listings TYPE datetime;
DEFINE FIELD images ON listings TYPE array;

-- Batch schema
DEFINE FIELD id ON batches TYPE string;
DEFINE FIELD status ON batches TYPE string;
DEFINE FIELD total ON batches TYPE number;
DEFINE FIELD processed ON batches TYPE number;
DEFINE FIELD created_at ON batches TYPE datetime;
DEFINE FIELD updated_at ON batches TYPE datetime;

-- Key schema
DEFINE FIELD id ON keys TYPE string;
DEFINE FIELD key ON keys TYPE string;
DEFINE FIELD status ON keys TYPE string;
DEFINE FIELD usage_count ON keys TYPE number;
DEFINE FIELD created_at ON keys TYPE datetime;
DEFINE FIELD expires_at ON keys TYPE datetime;

-- Indexes
DEFINE INDEX idx_listing_id ON images FIELDS listing_id;
DEFINE INDEX idx_b2_file_id ON images FIELDS b2_file_id;
DEFINE INDEX idx_created_at ON images FIELDS created_at;
DEFINE INDEX idx_processed ON images FIELDS processed;
DEFINE INDEX listing_status ON listings FIELDS status;
DEFINE INDEX batch_status ON batches FIELDS status;
DEFINE INDEX key_status ON keys FIELDS status;
DEFINE INDEX idx_job_status ON analysis_jobs FIELDS status;

-- Vector index on text_content
DEFINE INDEX idx_text_embedding ON images 
FIELDS text_content 
SEARCH ANALYZER ascii BM25 HIGHLIGHTS;

-- Add embedding field
DEFINE FIELD embedding ON images TYPE array;

-- Vector index for similarity search
DEFINE INDEX img_embedding ON images FIELDS embedding VECTOR 1536 COSINE;

-- Helper Functions for vector search
DEFINE FUNCTION fn::find_similar_images($text: string, $limit: int) {
    LET $embedding = ml::embed($text);
    RETURN SELECT 
        *,
        vector::similarity(embedding, $embedding) as similarity
    FROM images
    WHERE processed = true 
    ORDER BY similarity DESC
    LIMIT $limit;
};

-- Function to update image with embedded text
DEFINE FUNCTION fn::update_image_embedding($id: string, $text: string) {
    LET $embedding = ml::embed($text);
    RETURN UPDATE images 
    SET 
        embedding = $embedding,
        text_content = $text,
        processed = true,
        updated_at = time::now()
    WHERE id = $id;
}; 