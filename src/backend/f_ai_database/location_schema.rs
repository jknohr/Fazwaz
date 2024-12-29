use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::backend::common::types::listing_types::{GpsCoordinates, LocationDetails};

// Coordinates stored as [longitude, latitude]
#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinates(pub f64, pub f64);

// Represents a single administrative level (neighborhood, city, region, etc.)
#[derive(Debug, Serialize, Deserialize)]
pub struct AdministrativeLevel {
    pub level_type: String,  // e.g., "neighborhood", "district", "region"
    pub name: String,        // The actual name of this administrative division
    pub original_id: String, 
    // The original Mapbox ID
}

// Properties specific to the location
#[derive(Debug, Serialize, Deserialize)]
pub struct LocationProperties {
    pub accuracy: Option<String>,    // e.g., "rooftop"
    pub category: Option<String>,    // e.g., "building"
    pub address: Option<String>,     // Street address if available
    pub postcode: Option<String>,    // Postal/ZIP code
    pub maki: Option<String>,        // Mapbox icon identifier
}

// Main location record
#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub id: Thing,                              // SurrealDB record ID
    pub original_id: String,                    // Original Mapbox ID
    pub place_type: Vec<String>,                // Types of place
    pub formatted_address: String,              // Full formatted address
    pub coordinates: Coordinates,               // [longitude, latitude]
    pub properties: LocationProperties,         // Additional properties
    pub administrative_levels: Vec<AdministrativeLevel>, // Hierarchical context
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// Define the database schema
const SCHEMA: &str = r#"
-- Define the location table
DEFINE TABLE location SCHEMAFULL;

-- Define fields
DEFINE FIELD original_id ON location TYPE string;
DEFINE FIELD place_type ON location TYPE array;
DEFINE FIELD formatted_address ON location TYPE string;
DEFINE FIELD coordinates ON location TYPE array;
DEFINE FIELD properties.accuracy ON location TYPE option<string>;
DEFINE FIELD properties.category ON location TYPE option<string>;
DEFINE FIELD properties.address ON location TYPE option<string>;
DEFINE FIELD properties.postcode ON location TYPE option<string>;
DEFINE FIELD properties.maki ON location TYPE option<string>;
DEFINE FIELD administrative_levels ON location TYPE array;
DEFINE FIELD created_at ON location TYPE datetime;
DEFINE FIELD updated_at ON location TYPE datetime;

-- Define indexes
DEFINE INDEX location_original_id ON location FIELDS original_id UNIQUE;
DEFINE INDEX location_coordinates ON location FIELDS coordinates;
DEFINE INDEX location_postcode ON location FIELDS properties.postcode;

-- Geospatial index for coordinates
DEFINE INDEX location_geo ON location FIELDS coordinates TYPE geospatial;
"#;

// Example implementation for converting Mapbox data to Location
impl Location {
    fn from_mapbox(mapbox_data: serde_json::Value) -> Result<Self, Box<dyn std::error::Error>> {
        // Parse the Mapbox JSON response and convert it to our Location struct
        let center = mapbox_data["center"].as_array()
            .ok_or("Missing center coordinates")?;
        
        let administrative_levels = mapbox_data["context"].as_array()
            .map(|contexts| {
                contexts.iter().map(|ctx| AdministrativeLevel {
                    level_type: ctx["id"].as_str()
                        .unwrap_or("")
                        .split('.')
                        .next()
                        .unwrap_or("")
                        .to_string(),
                    name: ctx["text"].as_str()
                        .unwrap_or("")
                        .to_string(),
                    original_id: ctx["id"].as_str()
                        .unwrap_or("")
                        .to_string(),
                }).collect()
            })
            .unwrap_or_default();

        Ok(Location {
            id: Thing::from(("location", uuid::Uuid::new_v4().to_string())),
            original_id: mapbox_data["id"].as_str()
                .unwrap_or("")
                .to_string(),
            place_type: mapbox_data["place_type"].as_array()
                .map(|types| {
                    types.iter()
                        .filter_map(|t| t.as_str())
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default(),
            formatted_address: mapbox_data["place_name"].as_str()
                .unwrap_or("")
                .to_string(),
            coordinates: Coordinates(
                center[0].as_f64().unwrap_or(0.0),
                center[1].as_f64().unwrap_or(0.0)
            ),
            properties: LocationProperties {
                accuracy: mapbox_data["properties"]["accuracy"].as_str().map(String::from),
                category: mapbox_data["properties"]["category"].as_str().map(String::from),
                address: mapbox_data["properties"]["address"].as_str().map(String::from),
                postcode: mapbox_data["context"]
                    .as_array()
                    .and_then(|ctx| {
                        ctx.iter()
                            .find(|c| c["id"].as_str().map_or(false, |id| id.starts_with("postcode")))
                            .and_then(|p| p["text"].as_str().map(String::from))
                    }),
                maki: mapbox_data["properties"]["maki"].as_str().map(String::from),
            },
            administrative_levels,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        })
    }

    // Add method to convert to GpsCoordinates for listings
    pub fn to_gps_coordinates(&self) -> GpsCoordinates {
        GpsCoordinates {
            latitude: self.coordinates.1,  // Mapbox returns [lon, lat]
            longitude: self.coordinates.0,
        }
    }

    // Add method to get formatted location details
    pub fn to_location_details(&self) -> LocationDetails {
        let mut country = String::new();
        let mut province = String::new();
        let mut district = String::new();
        let mut subdistrict = String::new();

        // Parse administrative levels into our location structure
        for level in &self.administrative_levels {
            match AdminLevelType::from_mapbox_id(&level.level_type) {
                AdminLevelType::Country => country = level.name.clone(),
                AdminLevelType::Region => province = level.name.clone(),
                AdminLevelType::District => district = level.name.clone(),
                AdminLevelType::Suburb => subdistrict = level.name.clone(),
                _ => {}
            }
        }

        LocationDetails {
            country,
            province,
            district,
            subdistrict,
            unit_number: self.properties.address.clone(),
        }
    }
}

// Add an enum for administrative level types
#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum AdminLevelType {
    Country,
    Region,
    District,
    Place,
    Suburb,
    Neighborhood,
    Postcode,
    Address,
    Locality,
    Unknown(String), // For future-proofing
}

impl AdminLevelType {
    fn from_mapbox_id(id: &str) -> Self {
        match id.split('.').next().unwrap_or("") {
            "country" => AdminLevelType::Country,
            "region" => AdminLevelType::Region,
            "district" => AdminLevelType::District,
            "place" => AdminLevelType::Place,
            "suburb" => AdminLevelType::Suburb,
            "neighborhood" => AdminLevelType::Neighborhood,
            "postcode" => AdminLevelType::Postcode,
            "address" => AdminLevelType::Address,
            "locality" => AdminLevelType::Locality,
            other => AdminLevelType::Unknown(other.to_string()),
        }
    }
}

