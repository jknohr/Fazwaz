use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OwnershipType {
    Thai(ThaiOwnershipType),
    Cambodian(CambodianOwnershipType),
    Malaysian(MalaysianOwnershipType),
    UAE(UAEOwnershipType),
    Vietnam(VietnamOwnershipType),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LandTitleDeedType {
    Thai(ThaiLandTitleDeedType),
    Cambodian(CambodianLandTitleDeedType),
    Malaysian(MalaysianLandTitleDeedType),
    UAE(UAELandTitleDeedType),
    Vietnam(VietnamLandTitleDeedType),
}

// Move all country-specific ownership and land title deed types here 