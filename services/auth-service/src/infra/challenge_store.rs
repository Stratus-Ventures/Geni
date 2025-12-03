use dashmap::DashMap;
use async_trait::async_trait;
use crate::domain::ports::ChallengeStore;
use crate::domain::errors::ChallengeStoreError;
use tokio::time::{Duration, Instant};

// Simple in-memory store with basic TTL
// For production, this should be Redis
pub struct MemoryChallengeStore {
    // Map key -> (challenge, expires_at)
    store: DashMap<String, (String, Instant)>,
}

impl MemoryChallengeStore {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }

    // Clean up expired entries (could be run periodically)
    pub fn cleanup(&self) {
        let now = Instant::now();
        self.store.retain(|_, (_, expires_at)| *expires_at > now);
    }
}

#[async_trait]
impl ChallengeStore for MemoryChallengeStore {
    async fn store_challenge(
        &self,
        key: &str,
        challenge: &str,
        ttl_seconds: u64,
    ) -> Result<(), ChallengeStoreError> {
        let expires_at = Instant::now() + Duration::from_secs(ttl_seconds);
        self.store.insert(key.to_string(), (challenge.to_string(), expires_at));
        Ok(())
    }

    async fn get_challenge(&self, key: &str) -> Result<Option<String>, ChallengeStoreError> {
        if let Some(entry) = self.store.get(key) {
            let (challenge, expires_at) = entry.value();
            if *expires_at > Instant::now() {
                return Ok(Some(challenge.clone()));
            } else {
                // Lazy cleanup
                drop(entry);
                self.store.remove(key);
            }
        }
        Ok(None)
    }

    async fn delete_challenge(&self, key: &str) -> Result<(), ChallengeStoreError> {
        self.store.remove(key);
        Ok(())
    }
}
