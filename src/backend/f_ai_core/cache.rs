use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use tracing::{info, warn};

use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry<T> {
    pub value: T,
    pub expires_at: Instant,
}

pub struct Cache<T> 
where 
    T: Clone + Send + Sync + 'static 
{
    data: Arc<RwLock<HashMap<String, CacheEntry<T>>>>,
    ttl: Duration,
}

impl<T> Cache<T> 
where 
    T: Clone + Send + Sync + 'static 
{
    pub fn new(ttl_secs: u64) -> Self {
        Self {
            data: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    pub async fn get(&self, key: &str) -> Option<T> {
        let data = self.data.read().await;
        
        if let Some(entry) = data.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.value.clone());
            }
        }
        None
    }

    pub async fn set(&self, key: String, value: T) -> Result<()> {
        let mut data = self.data.write().await;
        
        data.insert(key.clone(), CacheEntry {
            value,
            expires_at: Instant::now() + self.ttl,
        });
        
        info!("Cached value for key: {}", key);
        Ok(())
    }

    pub async fn remove(&self, key: &str) -> Result<()> {
        let mut data = self.data.write().await;
        data.remove(key);
        info!("Removed cached value for key: {}", key);
        Ok(())
    }

    pub async fn clear(&self) -> Result<()> {
        let mut data = self.data.write().await;
        data.clear();
        info!("Cleared all cached values");
        Ok(())
    }

    pub async fn cleanup_expired(&self) -> Result<()> {
        let mut data = self.data.write().await;
        let now = Instant::now();
        
        data.retain(|_, entry| {
            let valid = entry.expires_at > now;
            if !valid {
                warn!("Removing expired cache entry");
            }
            valid
        });
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = Cache::<String>::new(1); // 1 second TTL
        
        // Test set and get
        cache.set("key1".to_string(), "value1".to_string()).await.unwrap();
        assert_eq!(cache.get("key1").await.unwrap(), "value1");
        
        // Test expiration
        sleep(Duration::from_secs(2)).await;
        assert!(cache.get("key1").await.is_none());
        
        // Test remove
        cache.set("key2".to_string(), "value2".to_string()).await.unwrap();
        cache.remove("key2").await.unwrap();
        assert!(cache.get("key2").await.is_none());
    }
} 