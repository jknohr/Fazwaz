use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::backend::common::{Result, AppError};
use crate::backend::common::types::id_types::ListingId;
use crate::backend::common::types::ownership_types::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ThaiLandTitleDeedType {
    Chanote,
    NorSor3Gor,
    NorSor3,
    NorSor2,
    SorKor1,
}

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateListingRequest {
    pub listing_id: ListingId,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub square_meter: u32,
    pub amenities: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateListingRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub bedrooms: Option<u32>,
    pub bathrooms: Option<u32>,
    pub square_meter: Option<u32>,
    pub amenities: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AvailableAs {
    ShortTerm,
    LongTerm,
    Sale,
    Both,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PropertyType {
    Apartment,
    Condominium,
    Townhouse,
    Villa,
    Land,
    Commercial,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PropertyFurnishing {
    FullyFurnished,
    PartiallyFurnished,
    Unfurnished,
    Negotiable,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PropertyCondition {
    New,
    AsNew,
    Good,
    Fair,
    Poor,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PropertyView {
    SeaView,
    MountainView,
    PoolView,
    GardenView,
    CityView,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ParkingSpots {
    Covered(u32),
    Open(u32),
    Valet(u32),
    Negotiable(u32),
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimeOfPurchase {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ListingStatus {
    Draft,
    Active,
    Inactive,
    Archived,
}

pub struct Listing {
    pub id: ListingId,
    pub title: String,
    pub description: String,
    pub property_type: PropertyType,
    pub ownership_type: ThaiOwnershipType,
    pub land_ownership_type: ThaiLandOwnershipType,
    pub asking_price: Option<f64>,
    pub long_term_rental: Option<f64>,
    pub short_term_rental: Option<f64>,
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub indoor_square_meters: u32,
    pub outdoor_square_meters: u32,
    pub plot_size: u32,
    pub unit_number: Option<String>,
    pub storeys: Option<String>,
    pub year_of_completion: Option<String>,
    pub amenities: Vec<String>,
    pub available_from: Option<DateTime<Utc>>,
    pub available_to: Option<DateTime<Utc>>,
    pub available_as: AvailableAs,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: ListingStatus,
    pub country: String,
    pub district: String,
    pub subdistrict: String,
}

impl Listing {
    pub fn validate(&self) -> Result<()> {
        if self.title.is_empty() {
            return Err(AppError::Validation("Title cannot be empty".into()).into());
        }
        
        if self.country.is_empty() || self.district.is_empty() || self.subdistrict.is_empty() {
            return Err(AppError::Validation("Location fields cannot be empty".into()).into());
        }

        Ok(())
    }

    pub fn location_path(&self) -> String {
        format!("{}/{}/{}", 
            self.country.to_lowercase(),
            self.district.to_lowercase(),
            self.subdistrict.to_lowercase()
        )
    }

    pub fn new(
        id: ListingId,
        title: String,
        description: String,
        property_type: PropertyType,
        ownership_type: ThaiOwnershipType,
        land_ownership_type: ThaiLandOwnershipType,
        asking_price: Option<f64>,
        bedrooms: u32,
        bathrooms: u32,
        indoor_square_meters: u32,
        outdoor_square_meters: u32,
        plot_size: u32,
        amenities: Vec<String>,
    ) -> Self {
        Self {
            id,
            title,
            description,
            property_type,
            ownership_type,
            land_ownership_type,
            asking_price,
            long_term_rental: None,
            short_term_rental: None,
            bedrooms,
            bathrooms,
            indoor_square_meters,
            outdoor_square_meters,
            plot_size,
            unit_number: None,
            storeys: None,
            year_of_completion: None,
            amenities,
            available_from: None,
            available_to: None,
            available_as: AvailableAs::Both,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            status: ListingStatus::Draft,
            country: String::new(),
            district: String::new(),
            subdistrict: String::new(),
        }
    }
} 


// Structure to hold all country-specific options
pub struct CountrySpecificOptions {
    pub currency: Currency,
    pub ownership_types: Vec<OwnershipType>,
    pub land_title_types: Vec<LandTitleDeedType>,
    pub provinces: Vec<Province>,
}       

pub struct Province {
    pub name: String,
    pub districts: Vec<District>,
}

pub struct District {
    pub name: String,
    pub subdistricts: Vec<String>,
}




#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ThaiOwnershipType {
    Leasehold,
    LeaseholdWithOptionToBuy,
    Company,
    ThaiFreehold,
    ForeignFreehold,
    BVICompany,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ThaiLandOwnershipType {
    Leasehold,
    LeaseholdWithOptionToBuy,
    Company,
    ThaiFreehold,
    BVICompany,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CambodianLandTitleDeedType {
    HardTitle,
    SoftTitle,
    StateLand,
    EconomicLandConcession,
    SocialLandConcession,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CambodianOwnershipType {
    Leasehold,
    Strata,
    HardTitleFreehold,
    CompanyOwnership,
    BVICompany,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MalaysianLandTitleDeedType {
    IndividualTitle,
    StrataTitle,
    MasterTitle,
    LeaseholdTitle,
    FreeholdTitle,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MalaysianOwnershipType {
    Leasehold99Years,
    Leasehold60Years,
    Freehold,
    BumiputraLot,
    MMTwoHCompany,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UAELandTitleDeedType {
    InitialSaleContract,
    OqoodRegistration,
    TitleDeed,
    MulkiyaRegistration,
    MusatahaRight,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UAEOwnershipType {
    Freehold,
    Leasehold,
    Musataha,
    Usufruct,
    Commonhold,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VietnamLandTitleDeedType {
    RedBook,
    PinkBook,
    LandUseCertificate,
    ConstructionPermit,
    InvestmentCertificate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum VietnamOwnershipType {
    LongTermLease,
    CompanyOwnership,
    LandUseRight,
    ResidentialHousing,
    BVICompany,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Country {
    Thailand,
    Cambodia,
    Malaysia,
    UAE,
    Vietnam,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Currency {
    THB,
    KHR,
    MYR,
    AED,
    VND,
}

fn localization_validator(country: Country) -> CountrySpecificOptions {
    match country {
        Country::Thailand => CountrySpecificOptions {
            currency: Currency::THB,
            ownership_types: vec![
                OwnershipType::Thai(ThaiOwnershipType::Leasehold),
                OwnershipType::Thai(ThaiOwnershipType::LeaseholdWithOptionToBuy),
                OwnershipType::Thai(ThaiOwnershipType::Company),
                OwnershipType::Thai(ThaiOwnershipType::ThaiFreehold),
                OwnershipType::Thai(ThaiOwnershipType::ForeignFreehold),
                OwnershipType::Thai(ThaiOwnershipType::BVICompany),
            ],
            land_title_types: vec![
                LandTitleDeedType::Thai(ThaiLandTitleDeedType::Chanote),
                LandTitleDeedType::Thai(ThaiLandTitleDeedType::NorSor3Gor),
                LandTitleDeedType::Thai(ThaiLandTitleDeedType::NorSor3),
                LandTitleDeedType::Thai(ThaiLandTitleDeedType::NorSor2),
                LandTitleDeedType::Thai(ThaiLandTitleDeedType::SorKor1),
            ],
            provinces: vec![
                Province {
                    name: "Bangkok".to_string(),
                    districts: vec![
                        District {
                            name: "Phra Nakhon".to_string(),
                            subdistricts: vec![
                                "Prah Borom Maha Ratchawang".to_string(),
                                "Wang Burapha Phirom".to_string(),
                                "Wat Ratchabophit".to_string(),
                                "Samran Rat".to_string(),
                                "San Chaopho Suea".to_string(),
                                "Sao Ching Cha".to_string(),
                                "Bowon Niwet".to_string(),
                                "Talat Yot".to_string(),
                                "Chana Songkhram".to_string(),
                                "Bang Khun Phrom".to_string(),
                                "Wat Sam Phraya".to_string(),
                            ],
                        },
                        District {
                            name: "Dusit".to_string(),
                            subdistricts: vec![
                                "Dusit".to_string(),
                                "Waschiraphayaban".to_string(),
                                "Suan Chitlada".to_string(),
                                "Si yaek maha nak".to_string(),
                                "thanon Nakhon Chai si".to_string(),
                            ],
                        },
                        District {
                            name: "Nong Chok".to_string(),
                            subdistricts: vec![
                                "Krathum rai".to_string(),
                                "nong chok".to_string(),
                                "khlong sip".to_string(),
                                "khlong sip song".to_string(),
                                "khok faet".to_string(),
                                "khu fang nuea".to_string(),
                                "lam phak chi".to_string(),
                                "lam toiting".to_string(),
                            ],
                        },
                        District {
                            name: "Bang Rak".to_string(),
                            subdistricts: vec![
                                "maha phruettharam".to_string(),
                                "si lom".to_string(),
                                "suriyawong".to_string(),
                                "bang rak".to_string(),
                                "si phraya".to_string(),
                            ],
                        },
                        District {
                            name: "Bang Khen".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bang Kapi".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Pathum Wan".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Pom Prap Sattru Phai".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Phra Khanong".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Min Buri".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Lat Krabang".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Yan Nawa".to_string(),
                            subdistricts: vec![],   
                        },
                        District {
                            name: "Samphanthawong".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Phaya Thai".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Thon Buri".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bangkok Yai".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Huai Khwang".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Khlong San".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Taling Chan".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bangkok Noi".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bang Khun Thian".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Phasi Charoen".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Nong Khaem".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Rat Burana".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bang Phlat".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Din Daeng".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bueng Kum".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Sathon".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bang Sue".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Chatuchak".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bang Kho Laem".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Prawet".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Khlong Toei".to_string() ,
                            subdistricts: vec![],
                        },
                        District {
                            name: "Suan Luang".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Chom Thong".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Don Mueang".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Ratchathewi".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Lat Phrao".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Watthana".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bang Khae".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Lak Si".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Sai Mai".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Khan Na Yao".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Saphan Sung".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Wang Thonglang".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Khlong Sam Wa".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bang Na".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Thawi Watthana".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Thung Khru".to_string(),
                            subdistricts: vec![],
                        },
                        District {
                            name: "Bang Bon".to_string(),
                            subdistricts: vec![],
                        },
                    ],
                },
            
                Province {
                    name: "Nonthaburi".to_string(),
                    districts: vec![
                        District {
                            name: "Nonthaburi".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Bangkok".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Chonburi".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Rayong".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
                Province {
                    name: "Songkhla".to_string(),
                    districts: vec![
                        District {
                            name: "Songkhla".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Hat Yai".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Phuket".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Phang Nga".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
                Province {
                    name: "Chonburi".to_string(),
                    districts: vec![
                        District {
                            name: "Chonburi".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Rayong".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Pattaya".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
                Province {
                    name: "Phuket".to_string(),
                    districts: vec![
                        District {
                            name: "mueang phuket".to_string(),
                            subdistricts: vec![
                                "talat yai".to_string(),
                                "talat nuea".to_string(),
                                "koh kaeo".to_string(),
                                "ratsada".to_string(),
                                "wichit".to_string(),
                                "chalong".to_string(),
                                "rawai".to_string(),
                                "karon".to_string(),
                            ],
                        },
                        District {
                            name: "kathu".to_string(),
                            subdistricts: vec![
                                "kathu".to_string(),
                                "patong".to_string(),
                                "kamala".to_string(),
                            ],
                        },
                        District {
                            name: "thalang".to_string(),
                            subdistricts: vec![
                                "Thep Krasattri".to_string(),
                                "Si Sunthon".to_string(),
                                "Choeng Thale".to_string(),
                                "Pakhlok".to_string(),
                                "Mai Khao".to_string(),
                                "Sakhu".to_string(),
                            ],
                        }
                    ],
                },
                Province {  
                    name: "Surat Thani".to_string(),
                    districts: vec![
                        District {
                            name: "Surat Thani".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Koh Samui".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Koh Phangan".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Koh Tao".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
                Province {
                    name: "Udon Thani".to_string(),
                    districts: vec![
                        District {
                            name: "Udon Thani".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Nong Bua Lamphu".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Mukdahan".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Mae Hong Son".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
                Province {
                    name: "Phitsanulok".to_string(),
                    districts: vec![
                        District {
                            name: "Phitsanulok".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Phichit".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Phrae".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Phayao".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
                Province {
                    name: "Khon Kaen".to_string(),
                    districts: vec![
                        District {
                            name: "Khon Kaen".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Ubon Ratchathani".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Maha Sarakham".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Nakhon Phanom".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
                Province {
                    name: "Nakhon Si".to_string(),
                    districts: vec![
                        District {
                            name: "Phayao".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Mae Hong Son".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Chiang Mai".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
                Province {
                    name: "Chiang Rai City".to_string(),
                    districts: vec![
                        District {
                            name: "Chiang Rai City".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Chiang Rai".to_string(),
                            subdistricts: vec![]
                        },
                        District {
                            name: "Mae Sai".to_string(),
                            subdistricts: vec![]
                        },
                    ],
                },
            ],
        },                                          
        Country::Cambodia => CountrySpecificOptions {
            currency: Currency::KHR,
            ownership_types: vec![
                OwnershipType::Cambodian(CambodianOwnershipType::Leasehold),
                OwnershipType::Cambodian(CambodianOwnershipType::Strata),
                OwnershipType::Cambodian(CambodianOwnershipType::HardTitleFreehold),
                OwnershipType::Cambodian(CambodianOwnershipType::CompanyOwnership),
                OwnershipType::Cambodian(CambodianOwnershipType::BVICompany),
            ],
            land_title_types: vec![
                LandTitleDeedType::Cambodian(CambodianLandTitleDeedType::HardTitle),
                LandTitleDeedType::Cambodian(CambodianLandTitleDeedType::SoftTitle),
                LandTitleDeedType::Cambodian(CambodianLandTitleDeedType::StateLand),
                LandTitleDeedType::Cambodian(CambodianLandTitleDeedType::EconomicLandConcession),
                LandTitleDeedType::Cambodian(CambodianLandTitleDeedType::SocialLandConcession),
            ],
            provinces: vec![
                Province {
                    name: "Phnom Penh".to_string(),
                    districts: vec![
                        District {
                            name: "Daun Penh".to_string(),
                            subdistricts: vec![]
                        }
                    ]
                }
            ],
        },
        Country::Malaysia => CountrySpecificOptions {
            currency: Currency::MYR,
            ownership_types: vec![OwnershipType::Malaysian(MalaysianOwnershipType::Freehold)],
            land_title_types: vec![LandTitleDeedType::Malaysian(MalaysianLandTitleDeedType::FreeholdTitle)],
            provinces: vec![],
        },
        Country::UAE => CountrySpecificOptions {
            currency: Currency::AED,
            ownership_types: vec![OwnershipType::UAE(UAEOwnershipType::Freehold)],
            land_title_types: vec![LandTitleDeedType::UAE(UAELandTitleDeedType::TitleDeed)],
            provinces: vec![],
        },
        Country::Vietnam => CountrySpecificOptions {
            currency: Currency::VND,
            ownership_types: vec![OwnershipType::Vietnam(VietnamOwnershipType::LongTermLease)],
            land_title_types: vec![LandTitleDeedType::Vietnam(VietnamLandTitleDeedType::RedBook)],
            provinces: vec![],
        },
    }   
}
