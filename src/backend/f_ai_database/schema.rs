use surrealdb::Surreal;
use surrealdb::engine::remote::ws::Client;
use crate::backend::common::error::error::Result;

pub async fn initialize_schema(client: &Surreal<Client>) -> Result<()> {
    init_metrics_schema(client).await?;
    init_listings_schema(client).await?;
    init_api_keys_schema(client).await?;
    init_audit_schema(client).await?;
    init_users_schema(client).await?;
    init_resources_schema(client).await?;
    init_temp_files_schema(client).await?;
    init_monitoring_schema(client).await?;
    init_images_schema(client).await?;
    Ok(())
}

async fn init_metrics_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE metrics SCHEMALESS;
        DEFINE FIELD metric_name ON metrics TYPE string ASSERT $value != NONE;
        DEFINE FIELD metric_type ON metrics TYPE string ASSERT $value != NONE;
        DEFINE FIELD value ON metrics TYPE number;
        DEFINE FIELD timestamp ON metrics TYPE datetime DEFAULT time::now();
        DEFINE INDEX idx_metrics ON metrics FIELDS metric_name, timestamp;
    "#).await?
        .check()?;
    Ok(())
}

async fn init_listings_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE listings SCHEMALESS;
        DEFINE FIELD title ON listings TYPE string ASSERT $value != NONE;
        DEFINE FIELD description ON listings TYPE string;
        DEFINE FIELD price ON listings TYPE number ASSERT $value >= 0;
        DEFINE FIELD currency ON listings TYPE string ASSERT $value != NONE;
        DEFINE FIELD location ON listings TYPE object;
        DEFINE FIELD location.country ON listings TYPE string ASSERT $value != NONE;
        DEFINE FIELD location.district ON listings TYPE string;
        DEFINE FIELD location.subdistrict ON listings TYPE string;
        DEFINE FIELD images ON listings TYPE array;
        DEFINE FIELD amenities ON listings TYPE array;
        DEFINE FIELD property_type ON listings TYPE string ASSERT $value != NONE;
        DEFINE FIELD status ON listings TYPE string ASSERT $value INSIDE ['draft', 'published', 'archived'];
        DEFINE FIELD created_by ON listings TYPE record(user) VALUE $auth.id;
        DEFINE FIELD created_at ON listings TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON listings TYPE datetime;
        DEFINE FIELD version ON listings TYPE object;
        DEFINE FIELD version.major ON listings TYPE number ASSERT $value >= 0;
        DEFINE FIELD version.minor ON listings TYPE number ASSERT $value >= 0;
        DEFINE FIELD version.patch ON listings TYPE number ASSERT $value >= 0;
        DEFINE INDEX idx_listings_status ON listings FIELDS status;
        DEFINE INDEX idx_listings_location ON listings FIELDS location.country, location.district;
        DEFINE INDEX idx_listings_price ON listings FIELDS price;
    "#).await?
        .check()?;
    Ok(())
}

async fn init_api_keys_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE api_keys SCHEMALESS;
        DEFINE FIELD key ON api_keys TYPE string ASSERT $value != NONE;
        DEFINE FIELD expires_at ON api_keys TYPE datetime;
        DEFINE FIELD permissions ON api_keys TYPE array;
        DEFINE FIELD scope ON api_keys TYPE string ASSERT $value != NONE;
        DEFINE INDEX idx_api_keys ON api_keys FIELDS key UNIQUE;
        DEFINE INDEX idx_expires ON api_keys FIELDS expires_at;
    "#).await?
        .check()?;
    Ok(())
}

async fn init_audit_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE audit_logs SCHEMALESS;
        DEFINE FIELD action ON audit_logs TYPE string ASSERT $value != NONE;
        DEFINE FIELD timestamp ON audit_logs TYPE datetime DEFAULT time::now();
        DEFINE FIELD user_id ON audit_logs TYPE string;
        DEFINE FIELD details ON audit_logs TYPE object;
        DEFINE FIELD resource_id ON audit_logs TYPE string;
        DEFINE INDEX idx_audit ON audit_logs FIELDS timestamp, action;
    "#).await?
        .check()?;
    Ok(())
}

async fn init_users_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE users SCHEMALESS;
        DEFINE FIELD username ON users TYPE string ASSERT $value != NONE;
        DEFINE FIELD role ON users TYPE string ASSERT $value INSIDE ['admin', 'user', 'readonly'];
        DEFINE FIELD last_login ON users TYPE datetime;
        DEFINE INDEX idx_username ON users FIELDS username UNIQUE;
    "#).await?
        .check()?;
    Ok(())
}

async fn init_resources_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE resources SCHEMALESS;
        DEFINE FIELD type ON resources TYPE string ASSERT $value INSIDE ['upload', 'processing', 'search', 'embedding'];
        DEFINE FIELD status ON resources TYPE string ASSERT $value INSIDE ['active', 'pending', 'completed', 'failed'];
        DEFINE FIELD created_at ON resources TYPE datetime DEFAULT time::now();
        DEFINE FIELD updated_at ON resources TYPE datetime;
        DEFINE FIELD metadata ON resources TYPE object;
        DEFINE INDEX idx_resources ON resources FIELDS type, status;
    "#).await?
        .check()?;
    Ok(())
}

async fn init_temp_files_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE temp_files SCHEMALESS;
        DEFINE FIELD path ON temp_files TYPE string ASSERT $value != NONE;
        DEFINE FIELD created_at ON temp_files TYPE datetime DEFAULT time::now();
        DEFINE FIELD expires_at ON temp_files TYPE datetime ASSERT $value > time::now();
        DEFINE FIELD cleanup_status ON temp_files TYPE string ASSERT $value INSIDE ['pending', 'in_progress', 'completed', 'failed'];
        DEFINE INDEX idx_temp_files_expiry ON temp_files FIELDS expires_at;
    "#).await?
        .check()?;
    Ok(())
}

async fn init_monitoring_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE monitoring SCHEMALESS;
        DEFINE FIELD component ON monitoring TYPE string ASSERT $value != NONE;
        DEFINE FIELD status ON monitoring TYPE string ASSERT $value INSIDE ['healthy', 'degraded', 'failed'];
        DEFINE FIELD last_check ON monitoring TYPE datetime DEFAULT time::now();
        DEFINE FIELD details ON monitoring TYPE object;
        DEFINE INDEX idx_monitoring ON monitoring FIELDS component, status;
    "#).await?
        .check()?;
    Ok(())
}

async fn init_images_schema(client: &Surreal<Client>) -> Result<()> {
    client.query(r#"
        DEFINE TABLE images SCHEMALESS;
        DEFINE FIELD original_path ON images TYPE string ASSERT $value != NONE;
        DEFINE FIELD processed_path ON images TYPE string;
        DEFINE FIELD watermarked_path ON images TYPE string;
        DEFINE FIELD mime_type ON images TYPE string ASSERT $value != NONE;
        DEFINE FIELD size ON images TYPE number;
        DEFINE FIELD dimensions ON images TYPE object;
        DEFINE FIELD dimensions.width ON images TYPE number;
        DEFINE FIELD dimensions.height ON images TYPE number;
        DEFINE FIELD metadata ON images TYPE object;
        DEFINE FIELD created_at ON images TYPE datetime DEFAULT time::now();
        DEFINE FIELD processed_at ON images TYPE datetime;
        DEFINE FIELD status ON images TYPE string ASSERT $value INSIDE ['pending', 'processing', 'completed', 'failed'];
        DEFINE INDEX idx_images_status ON images FIELDS status;
    "#).await?
        .check()?;
    Ok(())
}

// Copy all other init_*_schema functions from database.rs
// Keep the same implementation but change self.client to client parameter 