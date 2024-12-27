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
pub enum ThaiOwnershipType {
    Freehold,
    Leasehold,
    UsufructRight,
    SurfaceRight,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CambodianOwnershipType {
    HardTitle,
    SoftTitle,
    Leasehold,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MalaysianOwnershipType {
    Freehold,
    Leasehold,
    Malay_Reserve,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UAEOwnershipType {
    Freehold,
    Leasehold,
    Musataha,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VietnamOwnershipType {
    LandUseRight,
    Leasehold,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LandTitleDeedType {
    Thai(ThaiLandTitleDeedType),
    Cambodian(CambodianLandTitleDeedType),
    Malaysian(MalaysianLandTitleDeedType),
    UAE(UAELandTitleDeedType),
    Vietnam(VietnamLandTitleDeedType),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ThaiLandTitleDeedType {
    Chanote,
    NorSorSamKor,
    NorSorSam,
    PorBorTor,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CambodianLandTitleDeedType {
    HardTitle,
    SoftTitle,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MalaysianLandTitleDeedType {
    IndividualTitle,
    StrataTitle,
    MasterTitle,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UAELandTitleDeedType {
    StandardTitle,
    MusatahaTitle,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VietnamLandTitleDeedType {
    RedBook,
    PinkBook,
} 