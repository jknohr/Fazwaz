use serde::{Serialize, Deserialize};
use crate::backend::common::types::countries::CountryData;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThailandData {
    provinces: Vec<Province>,
    ownership_types: Vec<ThaiOwnershipType>,
    title_deed_types: Vec<ThaiLandTitleDeedType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ThaiOwnershipType {
    Leasehold,
    LeaseholdWithOptionToBuy,
    Company,
    ThaiFreehold,
    ForeignFreehold,
    BVICompany,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ThaiLandTitleDeedType {
    Chanote,
    NorSor3Gor,
    NorSor3,
    NorSor2,
    SorKor1,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Province {
    name: String,
    districts: Vec<District>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    name: String,
    subdistricts: Vec<String>,
}

impl ThailandData {
    pub fn new() -> Self {
        Self {
            provinces: load_thailand_provinces(),
            ownership_types: vec![
                ThaiOwnershipType::Leasehold,
                ThaiOwnershipType::LeaseholdWithOptionToBuy,
                ThaiOwnershipType::Company,
                ThaiOwnershipType::ThaiFreehold,
                ThaiOwnershipType::ForeignFreehold,
                ThaiOwnershipType::BVICompany,
            ],
            title_deed_types: vec![
                ThaiLandTitleDeedType::Chanote,
                ThaiLandTitleDeedType::NorSor3Gor,
                ThaiLandTitleDeedType::NorSor3,
                ThaiLandTitleDeedType::NorSor2,
                ThaiLandTitleDeedType::SorKor1,
            ],
        }
    }
}

impl CountryData for ThailandData {
    fn get_provinces(&self) -> Vec<String> {
        self.provinces.iter().map(|p| p.name.clone()).collect()
    }

    fn get_districts(&self, province: &str) -> Vec<String> {
        self.provinces.iter()
            .find(|p| p.name == province)
            .map(|p| p.districts.iter().map(|d| d.name.clone()).collect())
            .unwrap_or_default()
    }

    fn get_subdistricts(&self, province: &str, district: &str) -> Vec<String> {
        self.provinces.iter()
            .find(|p| p.name == province)
            .and_then(|p| p.districts.iter().find(|d| d.name == district))
            .map(|d| d.subdistricts.clone())
            .unwrap_or_default()
    }

    fn get_ownership_types(&self) -> Vec<String> {
        self.ownership_types.iter()
            .map(|ot| format!("{:?}", ot))
            .collect()
    }

    fn get_title_deed_types(&self) -> Vec<String> {
        self.title_deed_types.iter()
            .map(|tt| format!("{:?}", tt))
            .collect()
    }

    fn get_property_types(&self) -> Vec<String> {
        vec![
            "Condo".to_string(),
            "House".to_string(),
            "Villa".to_string(),
            "Land".to_string(),
            "Townhouse".to_string(),
            "Apartment".to_string(),
        ]
    }
}

fn load_thailand_provinces() -> Vec<Province> {
    vec![
        Province {
            name: "Bangkok".to_string(),
            districts: vec![
                District {
                    name: "Phra Nakhon".to_string(),
                    subdistricts: vec![
                        "Phra Borom Maha Ratchawang".to_string(),
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
                        "Wachiraphayaban".to_string(),
                        "Suan Chitlada".to_string(),
                        "Si Yaek Maha Nak".to_string(),
                        "Thanon Nakhon Chai Si".to_string(),
                    ],
                },
            ],
        },
        Province {
            name: "Phuket".to_string(),
            districts: vec![
                District {
                    name: "Mueang Phuket".to_string(),
                    subdistricts: vec![
                        "Talat Yai".to_string(),
                        "Talat Nuea".to_string(),
                        "Koh Kaeo".to_string(),
                        "Ratsada".to_string(),
                        "Wichit".to_string(),
                        "Chalong".to_string(),
                        "Rawai".to_string(),
                        "Karon".to_string(),
                    ],
                },
                District {
                    name: "Kathu".to_string(),
                    subdistricts: vec![
                        "Kathu".to_string(),
                        "Patong".to_string(),
                        "Kamala".to_string(),
                    ],
                },
                District {
                    name: "Thalang".to_string(),
                    subdistricts: vec![
                        "Thep Krasattri".to_string(),
                        "Si Sunthon".to_string(),
                        "Choeng Thale".to_string(),
                        "Pakhlok".to_string(),
                        "Mai Khao".to_string(),
                        "Sakhu".to_string(),
                    ],
                },
            ],
        },
    ]
} 