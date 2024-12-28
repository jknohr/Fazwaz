use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct CambodiaDetails {
    ownership_type: CambodianOwnershipType,
    land_title_type: CambodianLandTitleDeedType,
    province: CambodianProvince,
    district: String,
    commune: String,
}

#[derive(Debug, Serialize, Deserialize)]
enum CambodianOwnershipType {
    Leasehold,
    Strata,
    HardTitleFreehold,
    CompanyOwnership,
    BVICompany,
}

#[derive(Debug, Serialize, Deserialize)]
enum CambodianLandTitleDeedType {
    HardTitle,
    SoftTitle,
    StateLand,
    EconomicLandConcession,
    SocialLandConcession,
}

impl CambodiaDetails {
    fn get_currency() -> Currency {
        Currency::KHR
    }

    fn validate(&self) -> Result<(), String> {
        // Cambodia-specific validation rules
        Ok(())
    }
} 