
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::backend::common::types::{    
    listing_types::{ListingId, ListingLocation, ListingMetadata},
    id_types::{ImageId, ObjectId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserId(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhoneNumber(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyId(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub user_id: UserId,
    pub email: Email,
    pub secondary_email: Option<Email>,
    pub main_phone_number: PhoneNumber,
    pub secondary_phone_number: Option<PhoneNumber>,
    pub nationality: String,
    pub speaking_languages: Vec<String>,
    pub listings: Vec<ListingId>,
    pub api_keys: Vec<ApiKey>,
    pub properties: Vec<PropertyId>,
    pub agent_affiliation: Option<AgentAffiliation>,
    pub user_profile: UserProfile,
    pub created_at: DateTime<Utc>,
    pub last_login_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub first_name: String,
    pub last_name: String,
    pub social_profiles: SocialProfiles,
    pub gender: String,
    pub client_type: Option<Vec<ClientType>>,
    pub client_history: ClientHistory,
    pub client_details: ClientDetails,
    pub date_of_birth: DateTime<Utc>,
    pub profile_picture: Option<Image>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialProfiles {
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub whatsapp: Option<String>,
    pub tiktok: Option<String>,
    pub signal: Option<String>,
    pub telegram: Option<String>,
    pub snapchat: Option<String>,
    pub pinterest: Option<String>,
    pub youtube: Option<String>,
    pub reels: Option<String>,
    pub bluesky: Option<String>,
    pub twitter: Option<String>,
    pub linkedin: Option<String>,
    pub wechat: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientDetails {
    pub budget: Option<ClientBudget>,
    pub preferences: Option<Vec<ClientPreferences>>,
    pub family_status: Option<ClientFamilyStatus>,
    pub income: Option<ClientIncome>,
    pub property_preferences: Option<Vec<ClientPropertyType>>,
    pub searching_for: Option<Vec<ClientSearchingFor>>,
    pub occupation: Option<ClientOccupation>,
    pub price_range: Option<ClientPriceRange>,
    pub characteristics: Option<ClientCharacteristics>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAffiliation {
    pub agency_name: String,
    pub agency_id: String,
    pub role: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub url: String,
    pub alt_text: Option<String>,
    pub mime_type: String,
    pub size_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientHistory {
    pub interactions: Vec<ClientInteraction>,
    pub preferences: Vec<HistoricalPreference>,
    pub searches: Vec<SearchHistory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInteraction {
    pub timestamp: DateTime<Utc>,
    pub interaction_type: InteractionType,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Call,
    Email,
    Meeting,
    Viewing,
    Offer,
    Contract,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalPreference {
    pub timestamp: DateTime<Utc>,
    pub preference_type: PreferenceType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchHistory {
    pub timestamp: DateTime<Utc>,
    pub search_parameters: SearchParameters,
    pub results_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchParameters {
    pub location: Option<Vec<String>>,
    pub property_type: Option<Vec<PropertyType>>,
    pub price_range: Option<PriceRange>,
    pub other_filters: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientType {
    Buyer,
    Seller,
    Renter,
    Landlord,
    Investor,
    Agent,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BudgetType {
    Purchase,
    Rent,
    Investment,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PreferenceType {
    Location,
    PropertyType,
    Amenities,
    Style,
    ViewType,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FamilyStatus {
    Single,
    Married,
    Divorced,
    Widowed,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IncomeType {
    Salary,
    Business,
    Investment,
    Pension,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyType {
    Apartment,
    House,
    Villa,
    Townhouse,
    Land,
    Commercial,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    City,
    District,
    Neighborhood,
    Street,
    Building,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientSearchingFor {
    PrimaryResidence,
    SecondHome,
    Investment,
    Rental,
    Commercial,
    Other(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientOccupation {
    pub title: String,
    pub company: Option<String>,
    pub industry: Option<String>,
    pub years_experience: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientPriceRange {
    pub min: f64,
    pub max: f64,
    pub currency: String,
    pub flexibility_percentage: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCharacteristics {
    pub personality_traits: Option<Vec<String>>,
    pub communication_preferences: Option<Vec<String>>,
    pub decision_making_style: Option<String>,
    pub risk_tolerance: Option<String>,
    pub other_characteristics: Option<Vec<String>>,
    pub other_characteristics_description: Option<String>,
    pub preferred_timeslots: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRange {
    pub min_price: f64,
    pub max_price: f64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientPropertyLocation {
    pub location_type: LocationType,
    pub location_name: String,
    pub location_coordinates: GpsCoordinates,
    pub location_details: Option<LocationDetails>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientBudget {
    pub budget_type: Option<Vec<BudgetType>>,
    pub budget_amount: Option<Vec<f64>>,
    pub budget_currency: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientPreferences {
    pub preference_type: Option<Vec<PreferenceType>>,
    pub preference_value: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientFamilyStatus {
    pub family_status: Option<Vec<FamilyStatus>>,
    pub family_size: Option<Vec<u8>>,
    pub family_members: Option<Vec<String>>,
    pub family_income: Option<Vec<f64>>,
    pub family_income_currency: Option<Vec<String>>,
    pub family_location: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientIncome {
    pub income_type: Option<Vec<IncomeType>>,
    pub income_amount: Option<Vec<f64>>,
    pub income_currency: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientPropertyType {
    pub property_type: Option<Vec<PropertyType>>,
    pub property_location: Option<Vec<String>>,
    pub property_price: Option<Vec<f64>>,
    pub property_price_currency: Option<Vec<String>>,
    pub property_size: Option<Vec<f64>>,
    pub property_size_unit: Option<Vec<String>>,
}

impl UserId {
    pub fn new() -> Self {
        Self(uuid7::uuid7().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Email {
    pub fn new(email: String) -> Result<Self, String> {
        // Basic email validation
        if !email.contains('@') || !email.contains('.') {
            return Err("Invalid email format".to_string());
        }
        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PhoneNumber {
    pub fn new(number: String) -> Result<Self, String> {
        // Basic phone number validation
        if number.chars().any(|c| !c.is_ascii_digit() && c != '+' && c != ' ' && c != '-') {
            return Err("Invalid phone number format".to_string());
        }
        Ok(Self(number))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl User {
    pub fn new(
        email: Email,
        main_phone_number: PhoneNumber,
        nationality: String,
        first_name: String,
        last_name: String,
    ) -> Self {
        Self {
            user_id: UserId::new(),
            email,
            secondary_email: None,
            main_phone_number,
            secondary_phone_number: None,
            nationality,
            speaking_languages: Vec::new(),
            listings: Vec::new(),
            api_keys: Vec::new(),
            properties: Vec::new(),
            agent_affiliation: None,
            user_profile: UserProfile::new(first_name, last_name),
            created_at: Utc::now(),
            last_login_at: Utc::now(),
        }
    }

    pub fn add_api_key(&mut self, key: ApiKey) {
        self.api_keys.push(key);
    }

    pub fn add_listing(&mut self, listing_id: ListingId) {
        self.listings.push(listing_id);
    }

    pub fn add_property(&mut self, property_id: PropertyId) {
        self.properties.push(property_id);
    }

    pub fn update_last_login(&mut self) {
        self.last_login_at = Utc::now();
    }
}

impl UserProfile {
    pub fn new(first_name: String, last_name: String) -> Self {
        Self {
            first_name,
            last_name,
            social_profiles: SocialProfiles::default(),
            gender: String::new(),
            client_type: None,
            client_history: ClientHistory::new(),
            client_details: ClientDetails::default(),
            date_of_birth: Utc::now(), // Should be updated with actual DOB
            profile_picture: None,
        }
    }
}

impl Default for SocialProfiles {
    fn default() -> Self {
        Self {
            facebook: None,
            instagram: None,
            whatsapp: None,
            tiktok: None,
            signal: None,
            telegram: None,
            snapchat: None,
            pinterest: None,
            youtube: None,
            reels: None,
            bluesky: None,
            twitter: None,
            linkedin: None,
            wechat: None,
        }
    }
}

impl ClientHistory {
    pub fn new() -> Self {
        Self {
            interactions: Vec::new(),
            preferences: Vec::new(),
            searches: Vec::new(),
        }
    }

    pub fn add_interaction(&mut self, interaction_type: InteractionType, notes: Option<String>) {
        self.interactions.push(ClientInteraction {
            timestamp: Utc::now(),
            interaction_type,
            notes,
        });
    }

    pub fn add_search(&mut self, parameters: SearchParameters, results_count: u32) {
        self.searches.push(SearchHistory {
            timestamp: Utc::now(),
            search_parameters: parameters,
            results_count,
        });
    }
}

impl Default for ClientDetails {
    fn default() -> Self {
        Self {
            budget: None,
            preferences: None,
            family_status: None,
            income: None,
            property_preferences: None,
            searching_for: None,
            occupation: None,
            price_range: None,
            characteristics: None,
        }
    }
}

impl ClientPropertyLocation {
    pub fn new(
        location_type: LocationType,
        location_name: String,
        coordinates: GpsCoordinates,
        details: Option<LocationDetails>,
    ) -> Self {
        Self {
            location_type,
            location_name,
            location_coordinates: coordinates,
            location_details: details,
        }
    }
}

impl ApiKey {
    pub fn new() -> Self {
        use rand::{thread_rng, Rng};
        use base64::{Engine as _, engine::general_purpose::STANDARD};
        
        let mut rng = thread_rng();
        let mut bytes = [0u8; 32];
        rng.fill(&mut bytes);
        
        Self(STANDARD.encode(bytes))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl PropertyId {
    pub fn new() -> Self {
        Self(uuid7::uuid7().to_string())
    }

    pub fn from_string(s: String) -> Self {
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AgentAffiliation {
    pub fn new(agency_name: String, agency_id: String, role: String) -> Self {
        Self {
            agency_name,
            agency_id,
            role,
            start_date: Utc::now(),
            end_date: None,
        }
    }

    pub fn end_affiliation(&mut self) {
        self.end_date = Some(Utc::now());
    }
}

impl Image {
    pub fn new(url: String, mime_type: String, size_bytes: u64) -> Self {
        Self {
            url,
            alt_text: None,
            mime_type,
            size_bytes,
        }
    }

    pub fn with_alt_text(mut self, alt_text: String) -> Self {
        self.alt_text = Some(alt_text);
        self
    }
}

impl ClientPriceRange {
    pub fn new(min: f64, max: f64, currency: String) -> Result<Self, String> {
        if min > max {
            return Err("Minimum price cannot be greater than maximum price".to_string());
        }
        Ok(Self {
            min,
            max,
            currency,
            flexibility_percentage: None,
        })
    }

    pub fn with_flexibility(mut self, percentage: f32) -> Result<Self, String> {
        if !(0.0..=100.0).contains(&percentage) {
            return Err("Flexibility percentage must be between 0 and 100".to_string());
        }
        self.flexibility_percentage = Some(percentage);
        Ok(self)
    }

    pub fn get_range_with_flexibility(&self) -> (f64, f64) {
        match self.flexibility_percentage {
            Some(flex) => {
                let flex_amount = (self.max - self.min) * (flex as f64 / 100.0);
                (self.min - flex_amount, self.max + flex_amount)
            }
            None => (self.min, self.max),
        }
    }
}

impl ClientOccupation {
    pub fn new(title: String) -> Self {
        Self {
            title,
            company: None,
            industry: None,
            years_experience: None,
        }
    }

    pub fn with_company(mut self, company: String) -> Self {
        self.company = Some(company);
        self
    }

    pub fn with_industry(mut self, industry: String) -> Self {
        self.industry = Some(industry);
        self
    }

    pub fn with_experience(mut self, years: u8) -> Self {
        self.years_experience = Some(years);
        self
    }
}

impl ClientCharacteristics {
    pub fn new() -> Self {
        Self {
            personality_traits: None,
            communication_preferences: None,
            decision_making_style: None,
            risk_tolerance: None,
            other_characteristics: None,
            other_characteristics_description: None,
            preferred_timeslots: None,
        }
    }

    pub fn add_personality_trait(&mut self, trait_name: String) {
        self.personality_traits.get_or_insert_with(Vec::new).push(trait_name);
    }

    pub fn add_communication_preference(&mut self, preference: String) {
        self.communication_preferences.get_or_insert_with(Vec::new).push(preference);
    }

    pub fn set_decision_making_style(&mut self, style: String) {
        self.decision_making_style = Some(style);
    }

    pub fn set_risk_tolerance(&mut self, tolerance: String) {
        self.risk_tolerance = Some(tolerance);
    }
}

impl SearchParameters {
    pub fn new() -> Self {
        Self {
            location: None,
            property_type: None,
            price_range: None,
            other_filters: None,
        }
    }

    pub fn with_location(mut self, locations: Vec<String>) -> Self {
        self.location = Some(locations);
        self
    }

    pub fn with_property_type(mut self, types: Vec<PropertyType>) -> Self {
        self.property_type = Some(types);
        self
    }

    pub fn with_price_range(mut self, range: PriceRange) -> Self {
        self.price_range = Some(range);
        self
    }

    pub fn add_filter(&mut self, key: String, value: String) {
        self.other_filters.get_or_insert_with(HashMap::new).insert(key, value);
    }
}