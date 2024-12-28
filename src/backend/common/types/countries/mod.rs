use serde::{Serialize, Deserialize};
use anyhow::Result;

pub mod thailand;
pub mod cambodia;
pub mod uae;
pub mod malaysia;
pub mod vietnam;

#[derive(Debug, Serialize, Deserialize)]
pub enum Country {
    Thailand,
    Cambodia,
    UAE,
    Malaysia,
    Vietnam,
}

impl Country {
    pub fn load_country_data(&self) -> Result<Box<dyn CountryData>> {
        match self {
            Country::Thailand => Ok(Box::new(thailand::ThailandData::new())),
            Country::Cambodia => Ok(Box::new(cambodia::CambodiaData::new())),
            Country::UAE => Ok(Box::new(uae::UAEData::new())),
            Country::Malaysia => Ok(Box::new(malaysia::MalaysiaData::new())),
            Country::Vietnam => Ok(Box::new(vietnam::VietnamData::new())),
        }
    }
}

// Trait that all country data must implement
pub trait CountryData {
    fn get_provinces(&self) -> Vec<String>;
    fn get_districts(&self, province: &str) -> Vec<String>;
    fn get_subdistricts(&self, province: &str, district: &str) -> Vec<String>;
    fn get_ownership_types(&self) -> Vec<String>;
    fn get_title_deed_types(&self) -> Vec<String>;
    fn get_property_types(&self) -> Vec<String>;
}