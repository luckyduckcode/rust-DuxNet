use crate::core::data_structures::*;
use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use tracing::{debug, info};

pub struct DIDManager {
    pub secret_key: Vec<u8>, // Store only the secret key bytes
    pub keypair: SigningKey,
    pub did: DID,
}

impl DIDManager {
    pub fn new(endpoints: Vec<String>) -> Self {
        let mut rng = OsRng;
        let mut secret_bytes = [0u8; 32];
        use rand::RngCore;
        rng.fill_bytes(&mut secret_bytes);
        let keypair = SigningKey::from_bytes(&secret_bytes);
        let public_key = keypair.verifying_key().to_bytes().to_vec();
        let did_id = format!("did:duxnet:{}", hex::encode(&public_key[..16]));
        let did = DID {
            id: did_id,
            public_key,
            endpoints,
            created_at: get_current_timestamp(),
        };
        info!("Created new DID: {}", did.id);
        DIDManager { secret_key: keypair.to_bytes().to_vec(), keypair, did }
    }

    pub fn sign_message(&self, message: &[u8]) -> Vec<u8> {
        let signature = self.keypair.sign(message);
        signature.to_bytes().to_vec()
    }

    pub fn verify_signature(&self, sig_bytes: &[u8], message: &[u8]) -> Result<bool, ed25519_dalek::SignatureError> {
        use std::convert::TryInto;
        let sig: [u8; 64] = sig_bytes.try_into().map_err(|_| ed25519_dalek::SignatureError::new())?;
        let sig = ed25519_dalek::Signature::from_bytes(&sig);
        Ok(self.keypair.verify(message, &sig).is_ok())
    }

    pub fn get_did(&self) -> &DID {
        &self.did
    }

    pub fn get_public_key(&self) -> Vec<u8> {
        self.keypair.verifying_key().to_bytes().to_vec()
    }

    pub fn create_attestation(&self, target_did: String, score: f64, interaction_type: String) -> ReputationAttestation {
        let timestamp = get_current_timestamp();
        let message = format!("{}:{}:{}:{}", self.did.id, target_did, score, interaction_type);
        let signature = self.sign_message(message.as_bytes());
        
        ReputationAttestation {
            attester_did: self.did.id.clone(),
            target_did,
            score,
            interaction_type,
            timestamp,
            signature,
        }
    }

    pub fn verify_attestation(&self, attestation: &ReputationAttestation) -> bool {
        let message = format!("{}:{}:{}:{}", 
            attestation.attester_did, 
            attestation.target_did, 
            attestation.score, 
            attestation.interaction_type
        );
        
        // For now, we'll verify against our own public key
        // In a real implementation, you'd need to resolve the attester's DID
        self.verify_signature(
            &attestation.signature,
            message.as_bytes()
        ).unwrap_or(false)
    }

    pub fn sign_escrow_contract(&self, escrow_id: &str, state: &EscrowState) -> Vec<u8> {
        let message = format!("{}:{}", escrow_id, serde_json::to_string(state).unwrap());
        self.sign_message(message.as_bytes())
    }

    pub fn verify_escrow_signature(&self, escrow_id: &str, state: &EscrowState, signature: &[u8], public_key: &[u8]) -> bool {
        let message = format!("{}:{}", escrow_id, serde_json::to_string(state).unwrap());
        self.verify_signature(signature, message.as_bytes()).unwrap_or(false)
    }

    pub fn export_private_key(&self) -> Vec<u8> {
        self.secret_key.clone()
    }

    pub fn import_private_key(secret_key_bytes: Vec<u8>, endpoints: Vec<String>) -> Result<Self> {
        let secret_key: [u8; 32] = secret_key_bytes.clone().try_into().unwrap();
        let keypair = SigningKey::from_bytes(&secret_key);
        let public_key = keypair.verifying_key().to_bytes().to_vec();
        let did_id = format!("did:duxnet:{}", hex::encode(&public_key[..16]));
        let did = DID {
            id: did_id,
            public_key,
            endpoints,
            created_at: get_current_timestamp(),
        };
        info!("Imported DID: {}", did.id);
        Ok(DIDManager { secret_key: secret_key_bytes, keypair, did })
    }

    pub fn add_endpoint(&mut self, endpoint: String) {
        if !self.did.endpoints.contains(&endpoint) {
            self.did.endpoints.push(endpoint);
            debug!("Added endpoint to DID: {}", self.did.id);
        }
    }

    pub fn remove_endpoint(&mut self, endpoint: &str) {
        self.did.endpoints.retain(|e| e != endpoint);
        debug!("Removed endpoint from DID: {}", self.did.id);
    }

    pub fn update_endpoints(&mut self, endpoints: Vec<String>) {
        self.did.endpoints = endpoints;
        debug!("Updated endpoints for DID: {}", self.did.id);
    }
} 