pub mod data_structures;
pub mod dht;
pub mod identity;
pub mod reputation;
pub mod escrow;
pub mod tasks;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

use data_structures::*;
use dht::DHT;
use identity::DIDManager;
use reputation::ReputationSystem;
use escrow::EscrowManager;
use tasks::TaskEngine;
use crate::network::P2PNetwork;
use crate::wallet::Wallet;

pub struct DuxNetNode {
    pub node_id: NodeId,
    pub did_manager: DIDManager,
    pub dht: DHT,
    pub reputation_system: ReputationSystem,
    pub escrow_manager: EscrowManager,
    pub task_engine: TaskEngine,
    pub network: Arc<P2PNetwork>,
    pub wallet: Arc<RwLock<crate::wallet::Wallet>>,
    pub is_running: Arc<RwLock<bool>>,
}

impl DuxNetNode {
    pub async fn new(port: u16) -> Result<Self> {
        let node_id = NodeId(uuid::Uuid::new_v4().to_string());
        let endpoints = vec![format!("tcp://127.0.0.1:{}", port)];
        
        let did_manager = DIDManager::new(endpoints);
        let dht = DHT::new(node_id.clone());
        let reputation_system = ReputationSystem::new();
        let escrow_manager = EscrowManager::new();
        let task_engine = TaskEngine::new();
        let network = Arc::new(P2PNetwork::new(port).await?);
        let wallet = Arc::new(RwLock::new(crate::wallet::Wallet::new(did_manager.did.id.clone())?));
        let is_running = Arc::new(RwLock::new(false));
        
        Ok(DuxNetNode {
            node_id,
            did_manager,
            dht,
            reputation_system,
            escrow_manager,
            task_engine,
            network,
            wallet,
            is_running,
        })
    }

    pub async fn start(&mut self) -> Result<()> {
        info!("Starting DuxNet node: {}", self.node_id.0);
        
        // Start the P2P network
        self.network.start().await?;
        
        // Mark as running
        {
            let mut running = self.is_running.write().await;
            *running = true;
        }
        
        // Start the main event loop
        self.event_loop().await?;
        
        Ok(())
    }

    async fn event_loop(&self) -> Result<()> {
        loop {
            // Check if we should stop
            {
                let running = self.is_running.read().await;
                if !*running {
                    break;
                }
            }
            
            // Process network events
            if let Err(e) = self.network.process_events().await {
                error!("Network event processing error: {}", e);
            }
            
            // Process pending tasks
            if let Err(e) = self.task_engine.process_pending_tasks().await {
                error!("Task processing error: {}", e);
            }
            
            // Sleep briefly to prevent busy waiting
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        
        Ok(())
    }

    pub async fn stop(&self) -> Result<()> {
        info!("Stopping DuxNet node: {}", self.node_id.0);
        
        // Mark as stopped
        {
            let mut running = self.is_running.write().await;
            *running = false;
        }
        
        // Stop the network
        self.network.stop().await?;
        
        Ok(())
    }

    // Service management
    pub async fn register_service(&self, name: String, description: String, 
                                  price: u64) -> Result<ServiceId> {
        let service_id = ServiceId(uuid::Uuid::new_v4().to_string());
        let service = ServiceMetadata {
            id: service_id.clone(),
            provider_did: self.did_manager.did.id.clone(),
            name,
            description,
            endpoint: self.did_manager.did.endpoints[0].clone(),
            price,
            reputation_score: self.reputation_system.get_reputation(&self.did_manager.did.id).await,
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        self.dht.announce_service(&service).await?;
        info!("Registered service: {}", service_id.0);
        Ok(service_id)
    }

    pub async fn find_services(&self, query: &str) -> Vec<ServiceMetadata> {
        self.dht.find_services(query).await
    }

    // Escrow management
    pub async fn create_escrow_for_service(&self, service_id: &ServiceId, 
                                           seller_did: String, amount: u64) -> Result<String> {
        let arbiters = vec![
            "did:duxnet:arbiter1".to_string(),
            "did:duxnet:arbiter2".to_string(),
        ];
        
        let escrow_id = self.escrow_manager.create_escrow(
            self.did_manager.did.id.clone(),
            seller_did,
            arbiters,
            amount
        ).await?;
        
        info!("Created escrow: {}", escrow_id);
        Ok(escrow_id)
    }

    // Task management
    pub async fn submit_task(&self, service_id: ServiceId, payload: Vec<u8>, 
                             requirements: TaskRequirements) -> Result<TaskId> {
        let task_id = TaskId(uuid::Uuid::new_v4().to_string());
        let task = Task {
            id: task_id.clone(),
            escrow_id: "".to_string(), // Will be set when escrow is created
            service_id,
            payload,
            requirements,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        
        self.task_engine.submit_task(task).await?;
        info!("Submitted task: {}", task_id.0);
        Ok(task_id)
    }

    // Reputation management
    pub async fn get_reputation(&self, did: &str) -> f64 {
        self.reputation_system.get_reputation(did).await
    }

    pub async fn add_reputation_attestation(&self, attestation: ReputationAttestation) -> Result<()> {
        self.reputation_system.add_attestation(attestation).await?;
        Ok(())
    }
} 