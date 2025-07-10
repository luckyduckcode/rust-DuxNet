use crate::core::data_structures::*;
use anyhow::Result;
use ed25519_dalek::{SigningKey, VerifyingKey, Signature, Signer, Verifier};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Currency {
    BTC,    // Bitcoin
    ETH,    // Ethereum
    USDC,   // USD Coin
    LTC,    // Litecoin
    XMR,    // Monero
    DOGE,   // Dogecoin
}

impl Currency {
    pub fn symbol(&self) -> &'static str {
        match self {
            Currency::BTC => "BTC",
            Currency::ETH => "ETH",
            Currency::USDC => "USDC",
            Currency::LTC => "LTC",
            Currency::XMR => "XMR",
            Currency::DOGE => "DOGE",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Currency::BTC => "Bitcoin",
            Currency::ETH => "Ethereum",
            Currency::USDC => "USD Coin",
            Currency::LTC => "Litecoin",
            Currency::XMR => "Monero",
            Currency::DOGE => "Dogecoin",
        }
    }

    pub fn decimals(&self) -> u8 {
        match self {
            Currency::BTC => 8,
            Currency::ETH => 18,
            Currency::USDC => 6,
            Currency::LTC => 8,
            Currency::XMR => 12,
            Currency::DOGE => 8,
        }
    }

    pub fn initial_balance(&self) -> u64 {
        match self {
            Currency::BTC => 0,      // No free Bitcoin
            Currency::ETH => 0,      // No free Ethereum
            Currency::USDC => 1000,  // $1000 USDC
            Currency::LTC => 0,      // No free Litecoin
            Currency::XMR => 0,      // No free Monero
            Currency::DOGE => 10000, // 10,000 DOGE (much wow!)
        }
    }

    pub fn format_amount(&self, amount: u64) -> String {
        let decimals = self.decimals() as u32;
        let whole = amount / 10u64.pow(decimals);
        let fraction = amount % 10u64.pow(decimals);
        
        if fraction == 0 {
            format!("{} {}", whole, self.symbol())
        } else {
            let fraction_str = format!("{:0width$}", fraction, width = decimals as usize);
            format!("{}.{} {}", whole, fraction_str.trim_end_matches('0'), self.symbol())
        }
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wallet {
    pub did: String,
    pub secret_key: Vec<u8>, // Store only the secret key bytes
    pub balances: HashMap<Currency, u64>, // currency -> amount (in smallest units)
    pub transactions: Vec<Transaction>,
    pub preferred_currency: Currency,
    pub addresses: HashMap<Currency, String>, // currency -> address
    pub created_at: u64,
    pub last_activity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub currency: Currency,
    pub timestamp: u64,
    pub signature: Vec<u8>,
    pub status: TransactionStatus,
    pub fee: u64,
    pub block_height: Option<u64>,
    pub confirmations: u32,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    pub did: String,
    pub public_key: String,
    pub addresses: HashMap<String, String>, // currency -> address
    pub balances: HashMap<String, String>, // currency -> formatted balance
    pub total_transactions: usize,
    pub created_at: u64,
    pub last_activity: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendRequest {
    pub to_address: String,
    pub amount: u64,
    pub currency: Currency,
    pub memo: Option<String>,
    pub fee: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendResponse {
    pub transaction_id: String,
    pub success: bool,
    pub message: String,
    pub fee: u64,
}

impl Wallet {
    pub fn new(did: String) -> Result<Self> {
        let mut csprng = OsRng;
        let mut secret_bytes = [0u8; 32];
        use rand::RngCore;
        csprng.fill_bytes(&mut secret_bytes);
        let keypair = SigningKey::from_bytes(&secret_bytes);
        let secret_key_bytes = keypair.to_bytes().to_vec();
        
        let mut balances = HashMap::new();
        let mut addresses = HashMap::new();
        
        // Initialize balances and addresses for all supported currencies
        for currency in [Currency::BTC, Currency::ETH, Currency::USDC, Currency::LTC, Currency::XMR, Currency::DOGE] {
            balances.insert(currency, currency.initial_balance());
            addresses.insert(currency, Self::generate_address(&currency, &keypair.verifying_key()));
        }
        
        let now = get_current_timestamp();
        
        Ok(Wallet {
            did,
            secret_key: secret_key_bytes,
            balances,
            transactions: Vec::new(),
            preferred_currency: Currency::USDC, // Default to USDC for stability
            addresses,
            created_at: now,
            last_activity: now,
        })
    }

    pub fn set_preferred_currency(&mut self, currency: Currency) {
        self.preferred_currency = currency;
        self.last_activity = get_current_timestamp();
        info!("Set preferred currency to: {}", self.preferred_currency.name());
    }

    pub fn get_preferred_currency(&self) -> &Currency {
        &self.preferred_currency
    }

    pub fn get_keypair(&self) -> Result<SigningKey> {
        let secret_key: [u8; 32] = self.secret_key.clone().try_into().unwrap();
        Ok(SigningKey::from_bytes(&secret_key))
    }

    pub fn get_public_key(&self) -> Result<Vec<u8>> {
        let keypair = self.get_keypair()?;
        Ok(keypair.verifying_key().to_bytes().to_vec())
    }

    pub fn get_public_key_base64(&self) -> Result<String> {
        let public_key = self.get_public_key()?;
        Ok(general_purpose::STANDARD.encode(public_key))
    }

    pub fn get_private_key_base64(&self) -> Result<String> {
        Ok(general_purpose::STANDARD.encode(&self.secret_key))
    }

    pub fn get_address(&self, currency: &Currency) -> String {
        self.addresses.get(currency).cloned().unwrap_or_else(|| {
            // Generate address if not exists
            if let Ok(keypair) = self.get_keypair() {
                Self::generate_address(currency, &keypair.verifying_key())
            } else {
                "Invalid address".to_string()
            }
        })
    }

    pub fn get_all_addresses(&self) -> HashMap<Currency, String> {
        self.addresses.clone()
    }

    fn generate_address(currency: &Currency, public_key: &VerifyingKey) -> String {
        let pub_bytes = public_key.to_bytes();
        match currency {
            Currency::BTC => format!("1{}", hex::encode(&pub_bytes[..20])),
            Currency::ETH => format!("0x{}", hex::encode(&pub_bytes[..20])),
            Currency::USDC => format!("0x{}", hex::encode(&pub_bytes[..20])), // Same as ETH
            Currency::LTC => format!("L{}", hex::encode(&pub_bytes[..20])),
            Currency::XMR => format!("4{}", hex::encode(&pub_bytes[..32])),
            Currency::DOGE => format!("D{}", hex::encode(&pub_bytes[..20])),
        }
    }
    
    pub fn get_balance(&self, currency: &Currency) -> u64 {
        *self.balances.get(currency).unwrap_or(&0)
    }

    pub fn get_all_balances(&self) -> HashMap<Currency, u64> {
        self.balances.clone()
    }

    pub fn get_formatted_balance(&self, currency: &Currency) -> String {
        let amount = self.get_balance(currency);
        currency.format_amount(amount)
    }

    pub fn get_total_balance_usd(&self) -> f64 {
        // Simplified USD conversion rates (in real app, these would come from price feeds)
        let rates = HashMap::from([
            (Currency::BTC, 45000.0),
            (Currency::ETH, 3000.0),
            (Currency::USDC, 1.0),
            (Currency::LTC, 150.0),
            (Currency::XMR, 200.0),
            (Currency::DOGE, 0.08),
        ]);

        let mut total_usd = 0.0;
        for (currency, balance) in &self.balances {
            if let Some(&rate) = rates.get(currency) {
                let balance_f64 = *balance as f64 / 10f64.powi(currency.decimals() as i32);
                total_usd += balance_f64 * rate;
            }
        }
        total_usd
    }

    pub fn add_funds(&mut self, currency: Currency, amount: u64) {
        let current_balance = self.get_balance(&currency);
        let new_balance = current_balance + amount;
        self.balances.insert(currency.clone(), new_balance);
        self.last_activity = get_current_timestamp();
        info!("Added {} to wallet", currency.format_amount(amount));
    }

    pub fn remove_funds(&mut self, currency: &Currency, amount: u64) -> Result<()> {
        let current_balance = self.get_balance(currency);
        if current_balance < amount {
            return Err(anyhow::anyhow!("Insufficient balance"));
        }
        let new_balance = current_balance - amount;
        self.balances.insert(currency.clone(), new_balance);
        self.last_activity = get_current_timestamp();
        info!("Removed {} from wallet", currency.format_amount(amount));
        Ok(())
    }

    pub fn send_funds(&mut self, request: SendRequest) -> Result<SendResponse> {
        let current_balance = self.get_balance(&request.currency);
        let fee = request.fee.unwrap_or_else(|| self.calculate_fee(&request.currency));
        let total_amount = request.amount + fee;
        
        if current_balance < total_amount {
            return Err(anyhow::anyhow!("Insufficient balance. Need {} but have {}", 
                request.currency.format_amount(total_amount), 
                request.currency.format_amount(current_balance)));
        }
        
        let transaction_id = uuid::Uuid::new_v4().to_string();
        let timestamp = get_current_timestamp();
        
        let message = format!("{}:{}:{}:{}:{}:{}", 
            transaction_id, self.did, request.to_address, request.amount, 
            request.currency.symbol(), fee);
        
        let keypair = self.get_keypair()?;
        let signature = keypair.sign(message.as_bytes()).to_bytes().to_vec();
        
        let to_address_clone = request.to_address.clone();
        let transaction = Transaction {
            id: transaction_id.clone(),
            from: self.did.clone(),
            to: request.to_address,
            amount: request.amount,
            currency: request.currency.clone(),
            timestamp,
            signature,
            status: TransactionStatus::Pending,
            fee,
            block_height: None,
            confirmations: 0,
            memo: request.memo.clone(),
        };
        
        // Remove funds from wallet
        self.remove_funds(&request.currency, total_amount)?;
        
        // Add transaction to history
        self.transactions.push(transaction);
        self.last_activity = timestamp;
        
        info!("Sent {} to {}", request.currency.format_amount(request.amount), to_address_clone);
        
        Ok(SendResponse {
            transaction_id,
            success: true,
            message: "Transaction sent successfully".to_string(),
            fee,
        })
    }

    pub fn receive_funds(&mut self, from_address: String, amount: u64, currency: Currency, 
                        transaction_id: String, signature: Vec<u8>) -> Result<()> {
        let from_address_clone = from_address.clone();
        // Verify the transaction signature
                let _message = format!("{}:{}:{}:{}:{}",
            transaction_id, from_address, self.did, amount, currency.symbol());
        
        // In a real implementation, you'd verify the signature here
        // For now, we'll just accept it
        
        let transaction = Transaction {
            id: transaction_id,
            from: from_address,
            to: self.did.clone(),
            amount,
            currency: currency.clone(),
            timestamp: get_current_timestamp(),
            signature,
            status: TransactionStatus::Confirmed,
            fee: 0,
            block_height: Some(0), // Mock block height
            confirmations: 6, // Mock confirmations
            memo: None,
        };
        
        // Add funds to wallet
        self.add_funds(currency, amount);
        
        // Add transaction to history
        self.transactions.push(transaction);
        self.last_activity = get_current_timestamp();
        
        info!("Received {} from {}", currency.format_amount(amount), from_address_clone);
        Ok(())
    }

    pub fn calculate_fee(&self, currency: &Currency) -> u64 {
        // Simplified fee calculation (in real app, this would be dynamic)
        match currency {
            Currency::BTC => 1000, // 0.00001 BTC
            Currency::ETH => 21000000000000000, // 0.021 ETH
            Currency::USDC => 100000, // 0.1 USDC
            Currency::LTC => 10000, // 0.0001 LTC
            Currency::XMR => 1000000000, // 0.001 XMR
            Currency::DOGE => 1000000, // 0.01 DOGE
        }
    }
    
    pub fn create_transaction(&mut self, to: String, amount: u64, currency: Currency) -> Result<Transaction> {
        let current_balance = self.get_balance(&currency);
        if current_balance < amount {
            return Err(anyhow::anyhow!("Insufficient balance"));
        }
        
        let transaction_id = uuid::Uuid::new_v4().to_string();
        let timestamp = get_current_timestamp();
        
        let message = format!("{}:{}:{}:{}:{}", 
            transaction_id, self.did, to, amount, currency.symbol());
        
        let keypair = self.get_keypair()?;
        let signature = keypair.sign(message.as_bytes()).to_bytes().to_vec();
        
        let transaction = Transaction {
            id: transaction_id,
            from: self.did.clone(),
            to,
            amount,
            currency,
            timestamp,
            signature,
            status: TransactionStatus::Pending,
            fee: 0,
            block_height: None,
            confirmations: 0,
            memo: None,
        };
        
        Ok(transaction)
    }

    pub fn sign_transaction(&self, transaction: &mut Transaction) -> Result<()> {
        let message = format!("{}:{}:{}:{}:{}", 
            transaction.id, transaction.from, transaction.to, 
            transaction.amount, transaction.currency.symbol());
        
        let keypair = self.get_keypair()?;
        let signature = keypair.sign(message.as_bytes()).to_bytes().to_vec();
        transaction.signature = signature;
        transaction.timestamp = get_current_timestamp();
        
        Ok(())
    }

    pub fn verify_transaction(&self, transaction: &Transaction, public_key: &[u8]) -> bool {
        let message = format!("{}:{}:{}:{}:{}", 
            transaction.id, transaction.from, transaction.to, 
            transaction.amount, transaction.currency.symbol());
        
        if public_key.len() == 32 {
            if let Ok(verifying_key) = VerifyingKey::from_bytes(&public_key.try_into().unwrap()) {
                if transaction.signature.len() == 64 {
                    if let Ok(array) = <[u8; 64]>::try_from(&transaction.signature[..]) {
                        let signature = Signature::from_bytes(&array);
                        return verifying_key.verify(message.as_bytes(), &signature).is_ok();
                    }
                }
            }
        }
        false
    }

    pub fn process_transaction(&mut self, transaction: &Transaction) -> Result<()> {
        // Verify the transaction signature
        if !self.verify_transaction(transaction, &self.get_public_key()?) {
            return Err(anyhow::anyhow!("Invalid transaction signature"));
        }
        
        match transaction.status {
            TransactionStatus::Confirmed => {
                if transaction.to == self.did {
                    // We're receiving funds
                    self.add_funds(transaction.currency.clone(), transaction.amount);
                } else if transaction.from == self.did {
                    // We're sending funds
                    self.remove_funds(&transaction.currency, transaction.amount)?;
                }
            },
            TransactionStatus::Failed => {
                // Handle failed transaction (e.g., refund if we were the sender)
                if transaction.from == self.did {
                    // Refund the amount
                    self.add_funds(transaction.currency.clone(), transaction.amount);
                }
            },
            _ => {
                // Pending or other statuses - just add to history
            }
        }
        
        // Add to transaction history if not already present
        if !self.transactions.iter().any(|t| t.id == transaction.id) {
            self.transactions.push(transaction.clone());
        }
        
        self.last_activity = get_current_timestamp();
        Ok(())
    }

    pub fn get_transaction_history(&self) -> Vec<Transaction> {
        self.transactions.clone()
    }

    pub fn get_transactions_by_currency(&self, currency: &Currency) -> Vec<Transaction> {
        self.transactions.iter()
            .filter(|t| t.currency == *currency)
            .cloned()
            .collect()
    }

    pub fn get_transaction_by_id(&self, transaction_id: &str) -> Option<Transaction> {
        self.transactions.iter()
            .find(|t| t.id == transaction_id)
            .cloned()
    }

    pub fn get_wallet_info(&self) -> Result<WalletInfo> {
        let public_key = self.get_public_key_base64()?;
        let mut addresses = HashMap::new();
        let mut balances = HashMap::new();
        
        for currency in [Currency::BTC, Currency::ETH, Currency::USDC, Currency::LTC, Currency::XMR, Currency::DOGE] {
            addresses.insert(currency.symbol().to_string(), self.get_address(&currency));
            balances.insert(currency.symbol().to_string(), self.get_formatted_balance(&currency));
        }
        
        Ok(WalletInfo {
            did: self.did.clone(),
            public_key,
            addresses,
            balances,
            total_transactions: self.transactions.len(),
            created_at: self.created_at,
            last_activity: self.last_activity,
        })
    }

    pub fn export_private_key(&self) -> Result<Vec<u8>> {
        Ok(self.secret_key.clone())
    }

    pub fn export_private_key_base64(&self) -> Result<String> {
        Ok(self.get_private_key_base64()?)
    }

    pub fn import_private_key(secret_key_bytes: Vec<u8>, did: String) -> Result<Self> {
        let keypair = SigningKey::from_bytes(&secret_key_bytes.clone().try_into().unwrap());
        
        let mut balances = HashMap::new();
        let mut addresses = HashMap::new();
        
        // Initialize balances and addresses for all supported currencies
        for currency in [Currency::BTC, Currency::ETH, Currency::USDC, Currency::LTC, Currency::XMR, Currency::DOGE] {
            balances.insert(currency, currency.initial_balance());
            addresses.insert(currency, Self::generate_address(&currency, &keypair.verifying_key()));
        }
        
        let now = get_current_timestamp();
        
        Ok(Wallet {
            did,
            secret_key: secret_key_bytes,
            balances,
            transactions: Vec::new(),
            preferred_currency: Currency::USDC,
            addresses,
            created_at: now,
            last_activity: now,
        })
    }

    pub fn backup_wallet(&self) -> Result<String> {
        let wallet_data = serde_json::json!({
            "did": self.did,
            "secret_key": general_purpose::STANDARD.encode(&self.secret_key),
            "preferred_currency": self.preferred_currency,
            "created_at": self.created_at,
            "backup_version": "1.0"
        });
        
        Ok(wallet_data.to_string())
    }

    pub fn restore_wallet(backup_data: &str) -> Result<Self> {
        let wallet_data: serde_json::Value = serde_json::from_str(backup_data)?;
        
        let did = wallet_data["did"].as_str().unwrap().to_string();
        let secret_key_encoded = wallet_data["secret_key"].as_str().unwrap();
        let secret_key_bytes = general_purpose::STANDARD.decode(secret_key_encoded)?;
        
        Self::import_private_key(secret_key_bytes, did)
    }
}

// Multi-signature wallet for escrow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigWallet {
    pub address: String,
    pub participants: Vec<String>,
    pub threshold: usize,
    pub balance: u64,
    pub currency: Currency,
    pub pending_transactions: Vec<MultiSigTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiSigTransaction {
    pub id: String,
    pub to: String,
    pub amount: u64,
    pub currency: Currency,
    pub signatures: HashMap<String, Vec<u8>>,
    pub status: TransactionStatus,
    pub created_at: u64,
}

impl MultiSigWallet {
    pub fn new(participants: Vec<String>, threshold: usize, currency: Currency) -> Self {
        let address = format!("multisig_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
        
        MultiSigWallet {
            address,
            participants,
            threshold,
            balance: 0,
            currency,
            pending_transactions: Vec::new(),
        }
    }

    pub fn add_funds(&mut self, amount: u64) {
        self.balance += amount;
        info!("Added {} to multisig wallet", self.currency.format_amount(amount));
    }

    pub fn create_transaction(&mut self, to: String, amount: u64) -> Result<String> {
        if amount > self.balance {
            return Err(anyhow::anyhow!("Insufficient balance in multisig wallet"));
        }

        let transaction_id = uuid::Uuid::new_v4().to_string();
        let to_clone = to.clone();
        let transaction = MultiSigTransaction {
            id: transaction_id.clone(),
            to,
            amount,
            currency: self.currency.clone(),
            signatures: HashMap::new(),
            status: TransactionStatus::Pending,
            created_at: get_current_timestamp(),
        };

        self.pending_transactions.push(transaction);
        info!("Created multisig transaction: {} {} to {}", 
            self.currency.format_amount(amount), self.currency.symbol(), to_clone);

        Ok(transaction_id)
    }

    pub fn add_signature(&mut self, transaction_id: &str, signer: String, signature: Vec<u8>) -> Result<bool> {
        if let Some(transaction) = self.pending_transactions.iter_mut().find(|t| t.id == transaction_id) {
            if !self.participants.contains(&signer) {
                return Err(anyhow::anyhow!("Signer is not a participant"));
            }

            transaction.signatures.insert(signer, signature);

            // Check if we have enough signatures
            if transaction.signatures.len() >= self.threshold {
                transaction.status = TransactionStatus::Confirmed;
                self.balance -= transaction.amount;
                info!("Multisig transaction {} confirmed with {} signatures", 
                    transaction_id, transaction.signatures.len());
                return Ok(true);
            }
            Ok(false)
        } else {
            Err(anyhow::anyhow!("Transaction not found"))
        }
    }

    pub fn get_pending_transactions(&self) -> Vec<MultiSigTransaction> {
        self.pending_transactions.clone()
    }
} 