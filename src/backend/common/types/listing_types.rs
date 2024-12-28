use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use crate::backend::common::types::id_types::ListingId;
use crate::backend::common::types::countries::{
    thailand::ThailandDetails,
    cambodia::CambodiaDetails,
    uae::UAEDetails,
    malaysia::MalaysiaDetails,
    vietnam::VietnamDetails,
};

#[derive(Debug, Serialize, Deserialize)]
pub enum CountryDetails {
    Thailand(ThailandDetails),
    Cambodia(CambodiaDetails),
    UAE(UAEDetails),
    Malaysia(MalaysiaDetails),
    Vietnam(VietnamDetails),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    pub id: ListingId,
    pub fullname: String,
    pub phone: String,
    pub email: String,  
    pub api_key: String,
    pub title: String,
    pub description: String,
    pub property_type: PropertyType,
    pub country_details: CountryDetails,
    pub prices: PriceDetails,
    pub dimensions: PropertyDimensions,
    pub location: LocationDetails,
    pub amenities: Vec<String>,
    pub status: ListingStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
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
pub enum AvailableAs {
    ShortTerm,
    LongTerm,
    Sale,
    Both,
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
pub enum Currency {
    THB,
    KHR,
    MYR,
    AED,
    VND,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ListingStatus {
    Draft,
    Active,
    Inactive,
    Archived,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeOfPurchase {
    pub year: u32,
    pub month: u32,
    pub day: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Province {
    pub name: String,
    pub districts: Vec<District>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct District {
    pub name: String,
    pub subdistricts: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Money {
    pub amount: f64,
    pub currency: Currency,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PropertyDimensions {
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub indoor_square_meters: u32,
    pub outdoor_square_meters: u32,
    pub plot_size: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PriceDetails {
    pub asking_price: Option<Money>,
    pub long_term_rental: Option<Money>,
    pub short_term_rental: Option<Money>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LocationDetails {
    pub country: String,
    pub province: String,
    pub district: String,
    pub subdistrict: String,
    pub unit_number: Option<String>,
}
