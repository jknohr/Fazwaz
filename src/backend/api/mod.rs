pub mod health;
pub mod image;
pub mod key;
pub mod listing;
pub mod metrics;
pub mod search;
pub mod agent_listing_listener;
mod router;

pub use router::create_router;

// Common response types
pub mod response {
    use serde::Serialize;
    
    #[derive(Serialize)]
    pub struct MessageResponse {
        pub message: String,
    }

    #[derive(Serialize)]
    pub struct IdResponse {
        pub id: String,
    }
} 