use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

// Core identifiers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NodeId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ServiceId(pub String);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaskId(pub String);

// Decentralized Identity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DID {
    pub id: String,
    pub public_key: Vec<u8>,
    pub endpoints: Vec<String>,
    pub created_at: u64,
}

// Service metadata for DHT storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetadata {
    pub id: ServiceId,
    pub provider_did: String,
    pub name: String,
    pub description: String,
    pub endpoint: String,
    pub price: u64,
    pub reputation_score: f64,
    pub last_updated: u64,
}

// Reputation system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationAttestation {
    pub attester_did: String,
    pub target_did: String,
    pub score: f64,
    pub interaction_type: String,
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

// Escrow system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscrowContract {
    pub id: String,
    pub buyer_did: String,
    pub seller_did: String,
    pub arbiters: Vec<String>,
    pub amount: u64,
    pub state: EscrowState,
    pub multisig_address: String,
    pub signatures: HashMap<String, Vec<u8>>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EscrowState {
    Created,
    Funded,
    InProgress,
    Completed,
    Disputed,
    Refunded,
}

// Task system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub escrow_id: String,
    pub service_id: ServiceId,
    pub payload: Vec<u8>,
    pub requirements: TaskRequirements,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRequirements {
    pub cpu_cores: u32,
    pub memory_mb: u32,
    pub timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    pub task_id: TaskId,
    pub processor_did: String,
    pub result: Vec<u8>,
    pub proof: Vec<u8>,
    pub completed_at: u64,
}

// Network messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkMessage {
    // Service discovery
    ServiceAnnouncement(ServiceMetadata),
    ServiceQuery(String),
    ServiceResponse(Vec<ServiceMetadata>),
    
    // Task management
    TaskSubmission(Task),
    TaskAcceptance(TaskId, String), // task_id, processor_did
    TaskCompletion(TaskResult),
    
    // Escrow management
    EscrowCreation(EscrowContract),
    EscrowSignature(String, String, Vec<u8>), // escrow_id, signer_did, signature
    EscrowStateUpdate(String, EscrowState),
    
    // Reputation
    ReputationAttestation(ReputationAttestation),
    ReputationQuery(String), // target_did
    ReputationResponse(String, f64), // target_did, score
    
    // P2P ping/pong
    Ping,
    Pong,
}

// API request/response structures
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterServiceRequest {
    pub name: String,
    pub description: String,
    pub price: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterServiceResponse {
    pub service_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindServicesRequest {
    pub query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FindServicesResponse {
    pub services: Vec<ServiceMetadata>,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTaskRequest {
    pub service_id: String,
    pub payload: String,
    pub cpu_cores: u32,
    pub memory_mb: u32,
    pub timeout_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitTaskResponse {
    pub task_id: String,
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEscrowRequest {
    pub service_id: String,
    pub seller_did: String,
    pub amount: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEscrowResponse {
    pub escrow_id: String,
    pub success: bool,
    pub message: String,
}

// Node status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeStatus {
    pub node_id: String,
    pub did: String,
    pub is_online: bool,
    pub uptime_seconds: u64,
    pub services_count: usize,
    pub reputation_score: f64,
    pub peers_count: usize,
}

// Utility functions
pub fn get_current_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
} 