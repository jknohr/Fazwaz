use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMResponse {
    pub content: String,
    pub tokens_used: usize,
} 

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageAnalysis {
    pub photo_id: String,
    pub timestamp: String,
    pub location_context: LocationContext,
    pub primary_focus: String,
    pub area_details: AreaDetails,
    pub lighting_and_atmosphere: LightingAtmosphere,
    pub furniture_and_fixtures: FurnitureFixtures,
    pub outdoor_features: Option<OutdoorFeatures>,
    pub amenities_and_selling_points: AmenitiesSellingPoints,
    pub observations_and_issues: ObservationsIssues,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LocationContext {
    Indoor,
    Outdoor,
    Mixed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaDetails {
    pub area_type: String,
    pub size_category: SizeCategory,
    pub notable_features: Vec<String>,
    pub condition: AreaCondition,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SizeCategory {
    Small,
    Medium,
    Large,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AreaCondition {
    pub cleanliness: u8,  // 1-5
    pub damage: String,
    pub renovation_status: RenovationStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RenovationStatus {
    Modern,
    Dated,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LightingAtmosphere {
    pub natural_light_level: u8,  // 1-5
    pub artificial_light_level: u8,  // 1-5
    pub ambiance: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FurnitureFixtures {
    pub furniture_present: bool,
    pub furniture_type: Vec<String>,
    pub furniture_condition: FurnitureCondition,
    pub built_in_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OutdoorFeatures {
    pub outdoor_type: String,
    pub condition: u8,
    pub special_features: Vec<String>,
    pub view: View,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AmenitiesSellingPoints {
    pub visible_amenities: Vec<String>,
    pub decorative_elements: Vec<String>,
    pub standout_features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObservationsIssues {
    pub property_issues: String,
    pub potential_selling_points: Vec<String>,
    pub additional_notes: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct View {
    #[serde(rename = "type")]
    pub view_type: ViewType,
    pub quality: u8,
    pub obstructions: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ViewType {
    Nature,
    Urban,
    Mixed,
    Obstructed,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FurnitureCondition {
    New,
    Worn,
    Broken,
    Indeterminate,
}