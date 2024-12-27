use serde::{Deserialize, Serialize};
use uuid7;
use chrono::{DateTime, Utc};
use tracing::{info, warn, instrument};
use crate::backend::common::error::error::{Result, AppError};

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Hash)]
pub struct ListingId(String);

impl ListingId {
    #[instrument]
    pub fn new(value: String) -> Result<Self> {
        if Self::validate(&value) {
            info!(id = %value, "Created new ListingId");
            Ok(Self(value))
        } else {
            warn!(id = %value, "Invalid listing ID format");
            Err(AppError::Validation("Invalid listing ID format".into()))
        }
    }

    pub fn validate(value: &str) -> bool {
        !value.is_empty() && value.len() <= 50 
            && value.chars().all(|c| c.is_alphanumeric() || c == '-')
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Listing {
    pub id: String,  // Internal ID (UUID7)
    pub listing_id: ListingId,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub bedrooms: u32,
    pub bathrooms: u32,
    pub square_meter: u32,
    pub amenities: Vec<Amenity>,
    pub country: String,
    pub district: String,
    pub subdistrict: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: ListingStatus,
}

impl Listing {
    pub fn builder() -> ListingBuilder {
        ListingBuilder::default()
    }
    
    fn validate_price(price: f64) -> Result<()> {
        if price < 0.0 {
            return Err(AppError::Validation("Price cannot be negative".into()));
        }
        Ok(())
    }

    fn validate_rooms(rooms: u32) -> Result<()> {
        if rooms > 100 {
            return Err(AppError::Validation("Invalid number of rooms".into()));
        }
        Ok(())
    }

    fn validate_square_meter(sqft: u32) -> Result<()> {
        if sqft == 0 || sqft > 100_000 {
            return Err(AppError::Validation("Invalid square footage".into()));
        }
        Ok(())
    }

    fn validate_title(title: &str) -> Result<()> {
        if title.is_empty() || title.len() > 200 {
            return Err(AppError::Validation("Title must be between 1 and 200 characters".into()));
        }
        Ok(())
    }

    fn validate_description(description: &str) -> Result<()> {
        if description.len() > 2000 {
            return Err(AppError::Validation("Description cannot exceed 2000 characters".into()));
        }
        Ok(())
    }
    
    #[instrument(skip(self))]
    pub fn update_status(&mut self, new_status: ListingStatus) -> Result<()> {
        info!(
            listing_id = %self.listing_id.as_str(),
            from = ?self.status,
            to = ?new_status,
            "Attempting status transition"
        );

        match (&self.status, new_status) {
            (ListingStatus::Draft, ListingStatus::Active) => Ok(()),
            (ListingStatus::Active, ListingStatus::Inactive) => Ok(()),
            (ListingStatus::Inactive, ListingStatus::Active) => Ok(()),
            (_, ListingStatus::Archived) => Ok(()),
            _ => {
                warn!(
                    listing_id = %self.listing_id.as_str(),
                    from = ?self.status,
                    to = ?new_status,
                    "Invalid status transition"
                );
                Err(AppError::Validation("Invalid status transition".into()))
            }
        }?;
        
        info!(
            listing_id = %self.listing_id.as_str(),
            new_status = ?new_status,
            "Status updated"
        );
        self.status = new_status;
        self.updated_at = Utc::now();
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub enum ListingStatus {
    #[default]
    Draft,
    Active,
    Inactive,
    Archived,
} 

#[derive(Default)]
pub struct ListingBuilder {
    listing_id: Option<ListingId>,
    title: Option<String>,
    description: Option<String>,
    price: Option<f64>,
    bedrooms: Option<u32>,
    bathrooms: Option<u32>,
    square_meter: Option<u32>,
    amenities: Vec<Amenity>,
    country: Option<String>,
    district: Option<String>,
    subdistrict: Option<String>,
    status: Option<ListingStatus>,
}

impl ListingBuilder {
    pub fn listing_id(mut self, id: ListingId) -> Self {
        self.listing_id = Some(id);
        self
    }
    
    pub fn title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
    
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        self.price = Some(price);
        self
    }

    pub fn bedrooms(mut self, bedrooms: u32) -> Self {
        self.bedrooms = Some(bedrooms);
        self
    }

    pub fn bathrooms(mut self, bathrooms: u32) -> Self {
        self.bathrooms = Some(bathrooms);
        self
    }

    pub fn square_meter(mut self, square_meter: u32) -> Self {
        self.square_meter = Some(square_meter);
        self
    }

    pub fn amenities(mut self, amenities: Vec<Amenity>) -> Self {
        self.amenities = amenities;
        self
    }

    pub fn status(mut self, status: ListingStatus) -> Self {
        self.status = Some(status);
        self
    }

    pub fn country(mut self, country: String) -> Self {
        self.country = Some(country);
        self
    }

    pub fn district(mut self, district: String) -> Self {
        self.district = Some(district);
        self
    }

    pub fn subdistrict(mut self, subdistrict: String) -> Self {
        self.subdistrict = Some(subdistrict);
        self
    }

    pub fn build(self) -> Result<Listing> {
        let listing_id = self.listing_id.ok_or_else(|| 
            AppError::Validation("Listing ID is required".into()))?;
            
        let title = self.title.ok_or_else(|| 
            AppError::Validation("Title is required".into()))?;
        Listing::validate_title(&title)?;
            
        let description = self.description.unwrap_or_default();
        Listing::validate_description(&description)?;

        let price = self.price.ok_or_else(|| 
            AppError::Validation("Price is required".into()))?;
        Listing::validate_price(price)?;

        let bedrooms = self.bedrooms.unwrap_or(0);
        Listing::validate_rooms(bedrooms)?;

        let bathrooms = self.bathrooms.unwrap_or(0);
        Listing::validate_rooms(bathrooms)?;

        let square_meter = self.square_meter.unwrap_or(0);
        Listing::validate_square_meter(square_meter)?;
        
        Ok(Listing {
            id: uuid7::uuid7().to_string(),
            listing_id,
            title,
            description,
            price,
            bedrooms,
            bathrooms,
            square_meter,
            amenities: self.amenities,
            country: self.country.unwrap_or_default(),
            district: self.district.unwrap_or_default(),
            subdistrict: self.subdistrict.unwrap_or_default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            status: self.status.unwrap_or(ListingStatus::Draft),
        })
    }
} 

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum Amenity {
    Pool,
    Gym,
    Parking,
    AirConditioning,
    Furnished,
    PetFriendly,
    Laundry,
    Other(String),
}

impl Amenity {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pool => "Pool",
            Self::Gym => "Gym",
            Self::Parking => "Parking",
            Self::AirConditioning => "Air Conditioning",
            Self::Furnished => "Furnished",
            Self::PetFriendly => "Pet Friendly",
            Self::Laundry => "Laundry",
            Self::Other(s) => s,
        }
    }
} 