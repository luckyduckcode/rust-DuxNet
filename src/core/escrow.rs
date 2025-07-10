use crate::core::data_structures::*;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

pub struct EscrowManager {
    pub contracts: Arc<RwLock<HashMap<String, EscrowContract>>>,
    pub threshold: usize,
}

impl EscrowManager {
    pub fn new() -> Self {
        EscrowManager {
            contracts: Arc::new(RwLock::new(HashMap::new())),
            threshold: 2, // 2 out of 3 multisig by default
        }
    }

    pub async fn create_escrow(&self, buyer_did: String, seller_did: String, 
                               arbiters: Vec<String>, amount: u64) -> Result<String> {
        let escrow_id = uuid::Uuid::new_v4().to_string();
        let multisig_address = format!("multisig_{}", &escrow_id[..8]);
        
        let contract = EscrowContract {
            id: escrow_id.clone(),
            buyer_did,
            seller_did,
            arbiters,
            amount,
            state: EscrowState::Created,
            multisig_address,
            signatures: HashMap::new(),
            created_at: get_current_timestamp(),
        };
        
        let mut contracts = self.contracts.write().await;
        contracts.insert(escrow_id.clone(), contract);
        
        info!("Created escrow contract: {}", escrow_id);
        Ok(escrow_id)
    }

    pub async fn add_signature(&self, escrow_id: &str, signer_did: &str, 
                               signature: Vec<u8>) -> Result<()> {
        let mut contracts = self.contracts.write().await;
        if let Some(contract) = contracts.get_mut(escrow_id) {
            contract.signatures.insert(signer_did.to_string(), signature);
            
            // Check if we have enough signatures to proceed
            if contract.signatures.len() >= self.threshold {
                match contract.state {
                    EscrowState::Created => {
                        contract.state = EscrowState::Funded;
                        info!("Escrow {} funded with {} signatures", escrow_id, contract.signatures.len());
                    }
                    EscrowState::InProgress => {
                        contract.state = EscrowState::Completed;
                        info!("Escrow {} completed with {} signatures", escrow_id, contract.signatures.len());
                    }
                    _ => {}
                }
            }
            
            debug!("Added signature to escrow {} from {}", escrow_id, signer_did);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Escrow contract not found: {}", escrow_id))
        }
    }

    pub async fn get_contract(&self, escrow_id: &str) -> Option<EscrowContract> {
        let contracts = self.contracts.read().await;
        contracts.get(escrow_id).cloned()
    }

    pub async fn update_state(&self, escrow_id: &str, new_state: EscrowState) -> Result<()> {
        let mut contracts = self.contracts.write().await;
        if let Some(contract) = contracts.get_mut(escrow_id) {
            contract.state = new_state.clone();
            info!("Updated escrow {} state to {:?}", escrow_id, new_state);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Escrow contract not found: {}", escrow_id))
        }
    }

    pub async fn get_contracts_for_did(&self, did: &str) -> Vec<EscrowContract> {
        let contracts = self.contracts.read().await;
        contracts
            .values()
            .filter(|contract| {
                contract.buyer_did == did || 
                contract.seller_did == did || 
                contract.arbiters.contains(&did.to_string())
            })
            .cloned()
            .collect()
    }

    pub async fn get_pending_contracts(&self) -> Vec<EscrowContract> {
        let contracts = self.contracts.read().await;
        contracts
            .values()
            .filter(|contract| {
                matches!(contract.state, EscrowState::Created | EscrowState::Funded | EscrowState::InProgress)
            })
            .cloned()
            .collect()
    }

    pub async fn get_completed_contracts(&self) -> Vec<EscrowContract> {
        let contracts = self.contracts.read().await;
        contracts
            .values()
            .filter(|contract| {
                matches!(contract.state, EscrowState::Completed | EscrowState::Refunded)
            })
            .cloned()
            .collect()
    }

    pub async fn get_stats(&self) -> EscrowStats {
        let contracts = self.contracts.read().await;
        
        let mut stats = EscrowStats {
            total_contracts: contracts.len(),
            created: 0,
            funded: 0,
            in_progress: 0,
            completed: 0,
            disputed: 0,
            refunded: 0,
            total_amount: 0,
        };
        
        for contract in contracts.values() {
            match contract.state {
                EscrowState::Created => stats.created += 1,
                EscrowState::Funded => {
                    stats.funded += 1;
                    stats.total_amount += contract.amount;
                }
                EscrowState::InProgress => {
                    stats.in_progress += 1;
                    stats.total_amount += contract.amount;
                }
                EscrowState::Completed => {
                    stats.completed += 1;
                    stats.total_amount += contract.amount;
                }
                EscrowState::Disputed => {
                    stats.disputed += 1;
                    stats.total_amount += contract.amount;
                }
                EscrowState::Refunded => stats.refunded += 1,
            }
        }
        
        stats
    }
}

#[derive(Debug, Clone)]
pub struct EscrowStats {
    pub total_contracts: usize,
    pub created: usize,
    pub funded: usize,
    pub in_progress: usize,
    pub completed: usize,
    pub disputed: usize,
    pub refunded: usize,
    pub total_amount: u64,
} 