use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OwnershipType {
    // Asia
    pub afghanistan: (String, String, String),
    pub armenia: (String, String, String),
    pub azerbaijan: (String, String, String),
    pub bahrain: (String, String, String),
    pub bangladesh: (String, String, String),
    pub bhutan: (String, String, String),
    pub brunei: (String, String, String),
    pub cambodia: (String, String, String),
    pub china: (String, String, String),
    pub east_timor: (String, String, String),
    // ... add all other fields matching the implementation
    
    // The key is to add ALL country fields that are in the impl block
    // Each field should be declared as: pub country_name: (String, String, String)
}

// Rest of the implementation remains unchanged 