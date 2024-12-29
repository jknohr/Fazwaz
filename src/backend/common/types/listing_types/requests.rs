#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerListingRequest {
    pub fullname: String,
    pub email: String,
    pub phone_number: String,
    pub country: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnerListingResponse {
    pub listing_id: String,
    pub api_key: String,
} 