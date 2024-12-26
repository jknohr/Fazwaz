use tokio;
use anyhow::Result;

#[tokio::test]
async fn test_key_lifecycle() -> Result<()> {
    // Setup test database
    let db = setup_test_db().await?;
    
    // Test key generation
    let key_service = KeyService::new(db.clone());
    let key = key_service.generate_key("test@example.com", "testuser").await?;
    
    // Test key verification
    let metadata = key_service.verify_key(&key).await?;
    assert_eq!(metadata.username, "testuser");
    
    // Test key expiration
    // ... implement expiration test
    
    Ok(())
} 