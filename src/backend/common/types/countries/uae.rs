use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UAEDetails {
    pub ownership_type: UAEOwnershipType,
    pub land_title_type: UAELandTitleDeedType,
    pub emirate: Emirate,
    pub district: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UAEOwnershipType {
    Freehold,
    Leasehold,
    Musataha,
    Usufruct,
    Commonhold,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UAELandTitleDeedType {
    InitialSaleContract,
    OqoodRegistration,
    TitleDeed,
    MulkiyaRegistration,
    MusatahaRight,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Emirate {
    AbuDhabi,
    Dubai,
    Sharjah,
    Ajman,
    UmmAlQuwain,
    RasAlKhaimah,
    Fujairah,
}

impl UAEDetails {
    fn get_currency() -> Currency {
        Currency::AED
    }

    fn validate(&self) -> Result<(), String> {
        // UAE-specific validation rules
        Ok(())
    }
} 