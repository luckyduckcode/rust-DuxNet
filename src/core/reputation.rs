use crate::core::data_structures::*;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

pub struct ReputationSystem {
    pub attestations: Arc<RwLock<HashMap<String, Vec<ReputationAttestation>>>>,
    pub scores: Arc<RwLock<HashMap<String, f64>>>,
}

impl ReputationSystem {
    pub fn new() -> Self {
        ReputationSystem {
            attestations: Arc::new(RwLock::new(HashMap::new())),
            scores: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add_attestation(&self, attestation: ReputationAttestation) -> Result<()> {
        let mut attestations = self.attestations.write().await;
        attestations
            .entry(attestation.target_did.clone())
            .or_insert_with(Vec::new)
            .push(attestation.clone());
        
        self.recalculate_score(&attestation.target_did).await;
        debug!("Added reputation attestation for: {}", attestation.target_did);
        Ok(())
    }

    pub async fn get_reputation(&self, did: &str) -> f64 {
        let scores = self.scores.read().await;
        scores.get(did).copied().unwrap_or(0.0)
    }

    pub async fn get_attestations(&self, did: &str) -> Vec<ReputationAttestation> {
        let attestations = self.attestations.read().await;
        attestations.get(did).cloned().unwrap_or_default()
    }

    async fn recalculate_score(&self, did: &str) {
        let attestations = self.attestations.read().await;
        if let Some(atts) = attestations.get(did) {
            let now = get_current_timestamp();
            
            let mut weighted_sum = 0.0;
            let mut weight_sum = 0.0;
            
            for att in atts {
                // Apply time decay (older attestations have less weight)
                let age_days = (now - att.timestamp) / 86400;
                let decay_factor = 0.95_f64.powi(age_days as i32);
                let weight = decay_factor;
                
                weighted_sum += att.score * weight;
                weight_sum += weight;
            }
            
            let score = if weight_sum > 0.0 {
                weighted_sum / weight_sum
            } else {
                0.0
            };
            
            let mut scores = self.scores.write().await;
            scores.insert(did.to_string(), score);
            
            debug!("Recalculated reputation score for {}: {}", did, score);
        }
    }

    pub async fn remove_attestation(&self, target_did: &str, attester_did: &str, timestamp: u64) -> Result<()> {
        let mut attestations = self.attestations.write().await;
        if let Some(atts) = attestations.get_mut(target_did) {
            atts.retain(|att| {
                !(att.attester_did == attester_did && att.timestamp == timestamp)
            });
        }
        
        self.recalculate_score(target_did).await;
        debug!("Removed attestation for: {}", target_did);
        Ok(())
    }

    pub async fn get_top_nodes(&self, limit: usize) -> Vec<(String, f64)> {
        let scores = self.scores.read().await;
        let mut sorted_scores: Vec<(String, f64)> = scores
            .iter()
            .map(|(did, score)| (did.clone(), *score))
            .collect();
        
        sorted_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        sorted_scores.truncate(limit);
        
        sorted_scores
    }

    pub async fn get_stats(&self) -> ReputationStats {
        let attestations = self.attestations.read().await;
        let scores = self.scores.read().await;
        
        let total_attestations: usize = attestations.values().map(|v| v.len()).sum();
        let avg_score: f64 = if scores.is_empty() {
            0.0
        } else {
            scores.values().sum::<f64>() / scores.len() as f64
        };
        
        ReputationStats {
            total_nodes: scores.len(),
            total_attestations,
            average_score: avg_score,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ReputationStats {
    pub total_nodes: usize,
    pub total_attestations: usize,
    pub average_score: f64,
} 