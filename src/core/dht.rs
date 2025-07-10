use crate::core::data_structures::*;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

#[derive(Debug, Clone)]
pub struct DHTEntry {
    pub key: String,
    pub value: Vec<u8>,
    pub timestamp: u64,
    pub ttl: u64,
}

pub struct DHT {
    pub node_id: NodeId,
    pub entries: Arc<RwLock<HashMap<String, DHTEntry>>>,
    pub peers: Arc<RwLock<Vec<String>>>,
    pub k_bucket_size: usize,
}

impl DHT {
    pub fn new(node_id: NodeId) -> Self {
        DHT {
            node_id,
            entries: Arc::new(RwLock::new(HashMap::new())),
            peers: Arc::new(RwLock::new(Vec::new())),
            k_bucket_size: 20,
        }
    }

    pub async fn store(&self, key: String, value: Vec<u8>, ttl: u64) -> Result<()> {
        let entry = DHTEntry {
            key: key.clone(),
            value,
            ttl,
            timestamp: get_current_timestamp(),
        };
        
        let mut store = self.entries.write().await;
        store.insert(key.clone(), entry);
        debug!("Stored DHT entry: {}", key);
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Option<Vec<u8>> {
        let entries = self.entries.read().await;
        let now = get_current_timestamp();
        
        if let Some(entry) = entries.get(key) {
            if now < entry.timestamp + entry.ttl {
                debug!("Retrieved DHT entry: {}", key);
                return Some(entry.value.clone());
            } else {
                debug!("DHT entry expired: {}", key);
            }
        }
        None
    }

    pub async fn remove(&self, key: &str) -> Result<()> {
        let mut entries = self.entries.write().await;
        entries.remove(key);
        debug!("Removed DHT entry: {}", key);
        Ok(())
    }

    pub async fn announce_service(&self, service: &ServiceMetadata) -> Result<()> {
        let key = format!("service:{}", service.id.0);
        let value = serde_json::to_vec(service)?;
        self.store(key, value, 3600).await // 1 hour TTL
    }

    pub async fn find_services(&self, query: &str) -> Vec<ServiceMetadata> {
        let entries = self.entries.read().await;
        let mut services = Vec::new();
        
        for (key, entry) in entries.iter() {
            if key.starts_with("service:") {
                if let Ok(service) = serde_json::from_slice::<ServiceMetadata>(&entry.value) {
                    if service.name.to_lowercase().contains(&query.to_lowercase()) || 
                       service.description.to_lowercase().contains(&query.to_lowercase()) {
                        services.push(service);
                    }
                }
            }
        }
        
        debug!("Found {} services for query: {}", services.len(), query);
        services
    }

    pub async fn store_reputation_attestation(&self, attestation: &ReputationAttestation) -> Result<()> {
        let key = format!("reputation:{}:{}", attestation.target_did, attestation.timestamp);
        let value = serde_json::to_vec(attestation)?;
        self.store(key, value, 86400).await // 24 hour TTL
    }

    pub async fn get_reputation_attestations(&self, target_did: &str) -> Vec<ReputationAttestation> {
        let entries = self.entries.read().await;
        let mut attestations = Vec::new();
        
        for (key, entry) in entries.iter() {
            if key.starts_with(&format!("reputation:{}:", target_did)) {
                if let Ok(attestation) = serde_json::from_slice::<ReputationAttestation>(&entry.value) {
                    attestations.push(attestation);
                }
            }
        }
        
        debug!("Found {} reputation attestations for: {}", attestations.len(), target_did);
        attestations
    }

    pub async fn store_escrow_contract(&self, contract: &EscrowContract) -> Result<()> {
        let key = format!("escrow:{}", contract.id);
        let value = serde_json::to_vec(contract)?;
        self.store(key, value, 7200).await // 2 hour TTL
    }

    pub async fn get_escrow_contract(&self, escrow_id: &str) -> Option<EscrowContract> {
        let key = format!("escrow:{}", escrow_id);
        if let Some(value) = self.get(&key).await {
            serde_json::from_slice(&value).ok()
        } else {
            None
        }
    }

    pub async fn add_peer(&self, peer_id: String) -> Result<()> {
        let mut peers = self.peers.write().await;
        if !peers.contains(&peer_id) {
            peers.push(peer_id.clone());
            if peers.len() > self.k_bucket_size {
                peers.remove(0); // Remove oldest peer
            }
            debug!("Added peer: {}", peer_id);
        }
        Ok(())
    }

    pub async fn remove_peer(&self, peer_id: &str) -> Result<()> {
        let mut peers = self.peers.write().await;
        peers.retain(|p| p != peer_id);
        debug!("Removed peer: {}", peer_id);
        Ok(())
    }

    pub async fn get_peers(&self) -> Vec<String> {
        let peers = self.peers.read().await;
        peers.clone()
    }

    pub async fn cleanup_expired_entries(&self) -> Result<usize> {
        let mut entries = self.entries.write().await;
        let now = get_current_timestamp();
        let initial_count = entries.len();
        
        entries.retain(|_, entry| now < entry.timestamp + entry.ttl);
        
        let removed_count = initial_count - entries.len();
        if removed_count > 0 {
            debug!("Cleaned up {} expired DHT entries", removed_count);
        }
        
        Ok(removed_count)
    }

    pub async fn get_stats(&self) -> DHTStats {
        let entries = self.entries.read().await;
        let peers = self.peers.read().await;
        
        DHTStats {
            total_entries: entries.len(),
            total_peers: peers.len(),
            service_entries: entries.keys().filter(|k| k.starts_with("service:")).count(),
            reputation_entries: entries.keys().filter(|k| k.starts_with("reputation:")).count(),
            escrow_entries: entries.keys().filter(|k| k.starts_with("escrow:")).count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DHTStats {
    pub total_entries: usize,
    pub total_peers: usize,
    pub service_entries: usize,
    pub reputation_entries: usize,
    pub escrow_entries: usize,
} 