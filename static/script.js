// DuxNet Frontend JavaScript
const API_BASE = 'http://localhost:8081/api';

// Initialize the application
document.addEventListener('DOMContentLoaded', function() {
    loadNodeStatus();
    refreshStats();
    refreshBalances();
    loadWalletAddresses();
    
    // Auto-refresh stats every 30 seconds
    setInterval(refreshStats, 30000);
    
    // Auto-refresh balances every 60 seconds
    setInterval(refreshBalances, 60000);
});

// Load node status
async function loadNodeStatus() {
    try {
        const response = await fetch(`${API_BASE}/status`);
        const status = await response.json();
        
        document.getElementById('nodeDid').textContent = status.did;
        document.getElementById('nodeReputation').textContent = status.reputation_score.toFixed(2);
        document.getElementById('peerCount').textContent = status.peers_count;
        
    } catch (error) {
        console.error('Failed to load node status:', error);
        showNotification('Failed to load node status', 'error');
    }
}

// Register a new service
async function registerService() {
    const name = document.getElementById('serviceName').value;
    const description = document.getElementById('serviceDescription').value;
    const price = parseFloat(document.getElementById('servicePrice').value);
    const currency = document.getElementById('serviceCurrency').value;
    
    if (!name || !description || !price) {
        showNotification('Please fill in all fields', 'error');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/services/register`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                name,
                description,
                price,
                currency
            })
        });
        
        const result = await response.json();
        
        if (result.success) {
            showNotification(`Service registered successfully! ID: ${result.service_id}`, 'success');
            
            // Clear form
            document.getElementById('serviceName').value = '';
            document.getElementById('serviceDescription').value = '';
            document.getElementById('servicePrice').value = '';
            document.getElementById('serviceCurrency').value = 'USDC';
        } else {
            showNotification(result.message, 'error');
        }
        
    } catch (error) {
        console.error('Failed to register service:', error);
        showNotification('Failed to register service', 'error');
    }
}

// Search for services
async function searchServices() {
    const query = document.getElementById('searchQuery').value;
    
    if (!query) {
        showNotification('Please enter a search query', 'error');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/services/search`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                query
            })
        });
        
        const result = await response.json();
        
        if (result.success) {
            displaySearchResults(result.services);
            showNotification(`Found ${result.services.length} services`, 'success');
        } else {
            showNotification(result.message, 'error');
        }
        
    } catch (error) {
        console.error('Failed to search services:', error);
        showNotification('Failed to search services', 'error');
    }
}

// Display search results
function displaySearchResults(services) {
    const container = document.getElementById('searchResults');
    
    if (services.length === 0) {
        container.innerHTML = '<div class="result-item"><p>No services found</p></div>';
        return;
    }
    
    container.innerHTML = services.map(service => `
        <div class="result-item">
            <h4>${service.name}</h4>
            <p><strong>ID:</strong> ${service.id}</p>
            <p><strong>Description:</strong> ${service.description}</p>
            <p><strong>Price:</strong> ${service.price} ${service.currency || 'DOGE'}</p>
            <p><strong>Provider:</strong> ${service.provider_did}</p>
            <p><strong>Reputation:</strong> ${service.reputation_score.toFixed(2)}</p>
        </div>
    `).join('');
}

// Submit a task
async function submitTask() {
    const serviceId = document.getElementById('taskService').value;
    const payload = document.getElementById('taskPayload').value;
    const cpuCores = parseInt(document.getElementById('taskCpu').value);
    const memoryMb = parseInt(document.getElementById('taskMemory').value);
    const timeoutSeconds = parseInt(document.getElementById('taskTimeout').value);
    
    if (!serviceId || !payload) {
        showNotification('Please fill in service ID and payload', 'error');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/tasks/submit`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                service_id: serviceId,
                payload,
                cpu_cores: cpuCores,
                memory_mb: memoryMb,
                timeout_seconds: timeoutSeconds
            })
        });
        
        const result = await response.json();
        
        if (result.success) {
            showNotification(`Task submitted successfully! ID: ${result.task_id}`, 'success');
            
            // Clear form
            document.getElementById('taskService').value = '';
            document.getElementById('taskPayload').value = '';
            document.getElementById('taskCpu').value = '1';
            document.getElementById('taskMemory').value = '512';
            document.getElementById('taskTimeout').value = '60';
        } else {
            showNotification(result.message, 'error');
        }
        
    } catch (error) {
        console.error('Failed to submit task:', error);
        showNotification('Failed to submit task', 'error');
    }
}

// Create an escrow
async function createEscrow() {
    const serviceId = document.getElementById('escrowService').value;
    const sellerDid = document.getElementById('escrowSeller').value;
    const amount = parseFloat(document.getElementById('escrowAmount').value);
    const currency = document.getElementById('escrowCurrency').value;
    
    if (!serviceId || !sellerDid || !amount) {
        showNotification('Please fill in all fields', 'error');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/escrow/create`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                service_id: serviceId,
                seller_did: sellerDid,
                amount,
                currency
            })
        });
        
        const result = await response.json();
        
        if (result.success) {
            showNotification(`Escrow created successfully! ID: ${result.escrow_id}`, 'success');
            
            // Clear form
            document.getElementById('escrowService').value = '';
            document.getElementById('escrowSeller').value = '';
            document.getElementById('escrowAmount').value = '';
        } else {
            showNotification(result.message, 'error');
        }
        
    } catch (error) {
        console.error('Failed to create escrow:', error);
        showNotification('Failed to create escrow', 'error');
    }
}

// Refresh network statistics
async function refreshStats() {
    try {
        const response = await fetch(`${API_BASE}/stats`);
        const stats = await response.json();
        
        // Update DHT stats
        document.getElementById('dhtEntries').textContent = stats.dht.total_entries;
        document.getElementById('dhtPeers').textContent = stats.dht.total_peers;
        
        // Update reputation stats
        document.getElementById('repNodes').textContent = stats.reputation.total_nodes;
        document.getElementById('repAttestations').textContent = stats.reputation.total_attestations;
        
        // Update escrow stats
        document.getElementById('escrowContracts').textContent = stats.escrow.total_contracts;
        document.getElementById('escrowAmount').textContent = stats.escrow.total_amount;
        
        // Update task stats
        document.getElementById('taskPending').textContent = stats.tasks.pending_count;
        document.getElementById('taskCompleted').textContent = stats.tasks.completed_count;
        
    } catch (error) {
        console.error('Failed to refresh stats:', error);
    }
}

// Show notification
function showNotification(message, type) {
    const notification = document.getElementById('notification');
    notification.textContent = message;
    notification.className = `notification ${type}`;
    notification.classList.add('show');
    
    setTimeout(() => {
        notification.classList.remove('show');
    }, 3000);
}

// Utility function to simulate API call delay
async function simulateApiCall() {
    return new Promise(resolve => {
        setTimeout(resolve, 500 + Math.random() * 1000);
    });
}

// Refresh wallet balances
async function refreshBalances() {
    try {
        const response = await fetch(`${API_BASE}/wallet/balances`);
        const balances = await response.json();
        
        if (balances.success) {
            document.getElementById('btcBalance').textContent = formatCurrencyAmount(balances.balances.BTC || 0, 'BTC');
            document.getElementById('ethBalance').textContent = formatCurrencyAmount(balances.balances.ETH || 0, 'ETH');
            document.getElementById('usdcBalance').textContent = formatCurrencyAmount(balances.balances.USDC || 0, 'USDC');
            document.getElementById('ltcBalance').textContent = formatCurrencyAmount(balances.balances.LTC || 0, 'LTC');
            document.getElementById('xmrBalance').textContent = formatCurrencyAmount(balances.balances.XMR || 0, 'XMR');
            document.getElementById('dogeBalance').textContent = formatCurrencyAmount(balances.balances.DOGE || 0, 'DOGE');
        }
    } catch (error) {
        console.error('Failed to refresh balances:', error);
        // Don't show error notification for balance refresh to avoid spam
    }
}

// Change preferred currency
async function changePreferredCurrency() {
    const currency = document.getElementById('preferredCurrency').value;
    
    try {
        const response = await fetch(`${API_BASE}/wallet/preferred-currency`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                currency
            })
        });
        
        const result = await response.json();
        
        if (result.success) {
            showNotification(`Preferred currency changed to ${currency}`, 'success');
        } else {
            showNotification(result.message, 'error');
        }
    } catch (error) {
        console.error('Failed to change preferred currency:', error);
        showNotification('Failed to change preferred currency', 'error');
    }
}

// Format currency amount with proper decimals
function formatCurrencyAmount(amount, currency) {
    const decimals = {
        'BTC': 8,
        'ETH': 18,
        'USDC': 6,
        'LTC': 8,
        'XMR': 12,
        'DOGE': 8
    };
    
    const decimalPlaces = decimals[currency] || 8;
    const whole = Math.floor(amount / Math.pow(10, decimalPlaces));
    const fraction = amount % Math.pow(10, decimalPlaces);
    
    if (fraction === 0) {
        return `${whole} ${currency}`;
    } else {
        const fractionStr = fraction.toString().padStart(decimalPlaces, '0');
        return `${whole}.${fractionStr} ${currency}`;
    }
}

// Wallet Tab Management
function showWalletTab(tabName) {
    // Hide all tab contents
    const tabContents = document.querySelectorAll('.tab-content');
    tabContents.forEach(content => content.classList.remove('active'));
    
    // Remove active class from all tab buttons
    const tabButtons = document.querySelectorAll('.tab-btn');
    tabButtons.forEach(btn => btn.classList.remove('active'));
    
    // Show selected tab content
    document.getElementById(`${tabName}-tab`).classList.add('active');
    
    // Add active class to clicked button
    event.target.classList.add('active');
    
    // Load tab-specific data
    switch(tabName) {
        case 'balances':
            refreshBalances();
            break;
        case 'receive':
            loadWalletAddresses();
            break;
        case 'history':
            refreshTransactionHistory();
            break;
        case 'keys':
            refreshKeys();
            break;
    }
}

// Enhanced Balance Refresh
async function refreshBalances() {
    try {
        const response = await fetch(`${API_BASE}/wallet/balances`);
        const result = await response.json();
        
        if (result.success) {
            // Update individual balances
            document.getElementById('btcBalance').textContent = result.balances.BTC || '0.00000000 BTC';
            document.getElementById('ethBalance').textContent = result.balances.ETH || '0.000000000000000000 ETH';
            document.getElementById('usdcBalance').textContent = result.balances.USDC || '0.000000 USDC';
            document.getElementById('ltcBalance').textContent = result.balances.LTC || '0.00000000 LTC';
            document.getElementById('xmrBalance').textContent = result.balances.XMR || '0.000000000000 XMR';
            document.getElementById('dogeBalance').textContent = result.balances.DOGE || '0.00000000 DOGE';
            
            // Update total USD value
            document.getElementById('totalUsdValue').textContent = `$${result.total_usd.toFixed(2)} USD`;
            
            showNotification('Balances refreshed successfully', 'success');
        } else {
            showNotification(result.message, 'error');
        }
    } catch (error) {
        console.error('Failed to refresh balances:', error);
        showNotification('Failed to refresh balances', 'error');
    }
}

// Load Wallet Addresses
async function loadWalletAddresses() {
    try {
        const response = await fetch(`${API_BASE}/wallet/addresses`);
        const result = await response.json();
        
        if (result.success) {
            document.getElementById('btcAddress').textContent = result.addresses.BTC || 'Address not available';
            document.getElementById('ethAddress').textContent = result.addresses.ETH || 'Address not available';
            document.getElementById('usdcAddress').textContent = result.addresses.USDC || 'Address not available';
            document.getElementById('ltcAddress').textContent = result.addresses.LTC || 'Address not available';
            document.getElementById('xmrAddress').textContent = result.addresses.XMR || 'Address not available';
            document.getElementById('dogeAddress').textContent = result.addresses.DOGE || 'Address not available';
        } else {
            showNotification(result.message, 'error');
        }
    } catch (error) {
        console.error('Failed to load addresses:', error);
        showNotification('Failed to load addresses', 'error');
    }
}

// Copy Address to Clipboard
async function copyAddress(elementId) {
    const address = document.getElementById(elementId).textContent;
    try {
        await navigator.clipboard.writeText(address);
        showNotification('Address copied to clipboard!', 'success');
    } catch (error) {
        console.error('Failed to copy address:', error);
        showNotification('Failed to copy address', 'error');
    }
}

// Send Funds
async function sendFunds() {
    const toAddress = document.getElementById('sendToAddress').value;
    const amount = parseFloat(document.getElementById('sendAmount').value);
    const currency = document.getElementById('sendCurrency').value;
    const memo = document.getElementById('sendMemo').value;
    
    if (!toAddress || !amount || amount <= 0) {
        showNotification('Please enter a valid address and amount', 'error');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/wallet/send`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                to_address: toAddress,
                amount: Math.floor(amount * Math.pow(10, getCurrencyDecimals(currency))),
                currency: currency,
                memo: memo || null
            })
        });
        
        const result = await response.json();
        
        if (result.success) {
            showNotification(`Transaction sent! ID: ${result.transaction_id}`, 'success');
            
            // Clear form
            document.getElementById('sendToAddress').value = '';
            document.getElementById('sendAmount').value = '';
            document.getElementById('sendMemo').value = '';
            
            // Refresh balances
            refreshBalances();
            refreshTransactionHistory();
        } else {
            showNotification(result.message, 'error');
        }
    } catch (error) {
        console.error('Failed to send funds:', error);
        showNotification('Failed to send funds', 'error');
    }
}

// Get currency decimals
function getCurrencyDecimals(currency) {
    const decimals = {
        'BTC': 8,
        'ETH': 18,
        'USDC': 6,
        'LTC': 8,
        'XMR': 12,
        'DOGE': 8
    };
    return decimals[currency] || 8;
}

// Refresh Transaction History
async function refreshTransactionHistory() {
    try {
        const response = await fetch(`${API_BASE}/wallet/transactions`);
        const result = await response.json();
        
        if (result.success) {
            displayTransactionHistory(result.transactions);
        } else {
            showNotification(result.message, 'error');
        }
    } catch (error) {
        console.error('Failed to load transaction history:', error);
        showNotification('Failed to load transaction history', 'error');
    }
}

// Display Transaction History
function displayTransactionHistory(transactions) {
    const container = document.getElementById('transactionHistory');
    
    if (transactions.length === 0) {
        container.innerHTML = '<p>No transactions yet</p>';
        return;
    }
    
    container.innerHTML = transactions.map(tx => `
        <div class="transaction-item">
            <h4>Transaction ${tx.id.substring(0, 8)}...</h4>
            <div class="transaction-details">
                <div>
                    <p><strong>From:</strong> ${tx.from.substring(0, 20)}...</p>
                    <p><strong>To:</strong> ${tx.to.substring(0, 20)}...</p>
                    <p><strong>Amount:</strong> ${formatCurrencyAmount(tx.amount, tx.currency)}</p>
                </div>
                <div>
                    <p><strong>Date:</strong> ${new Date(tx.timestamp * 1000).toLocaleString()}</p>
                    <p><strong>Fee:</strong> ${formatCurrencyAmount(tx.fee, tx.currency)}</p>
                    <span class="transaction-status status-${tx.status.toLowerCase()}">${tx.status}</span>
                </div>
            </div>
            ${tx.memo ? `<p><strong>Memo:</strong> ${tx.memo}</p>` : ''}
        </div>
    `).join('');
}

// Filter Transactions
function filterTransactions() {
    const currency = document.getElementById('historyCurrency').value;
    // In a real implementation, you'd filter the transactions here
    // For now, just refresh the history
    refreshTransactionHistory();
}

// Refresh Keys
async function refreshKeys() {
    try {
        const response = await fetch(`${API_BASE}/wallet/keys`);
        const result = await response.json();
        
        if (result.success) {
            document.getElementById('publicKey').textContent = result.public_key;
            document.getElementById('privateKey').textContent = result.private_key;
        } else {
            showNotification(result.message, 'error');
        }
    } catch (error) {
        console.error('Failed to load keys:', error);
        showNotification('Failed to load keys', 'error');
    }
}

// Copy Key to Clipboard
async function copyKey(elementId) {
    const key = document.getElementById(elementId).textContent;
    try {
        await navigator.clipboard.writeText(key);
        showNotification('Key copied to clipboard!', 'success');
    } catch (error) {
        console.error('Failed to copy key:', error);
        showNotification('Failed to copy key', 'error');
    }
}

// Backup Wallet
async function backupWallet() {
    try {
        const response = await fetch(`${API_BASE}/wallet/backup`);
        const result = await response.json();
        
        if (result.success) {
            document.getElementById('backupText').value = result.backup_data;
            document.getElementById('backupData').style.display = 'block';
            showNotification('Wallet backup created successfully', 'success');
        } else {
            showNotification(result.message, 'error');
        }
    } catch (error) {
        console.error('Failed to backup wallet:', error);
        showNotification('Failed to backup wallet', 'error');
    }
}

// Copy Backup to Clipboard
async function copyBackup() {
    const backupText = document.getElementById('backupText').value;
    try {
        await navigator.clipboard.writeText(backupText);
        showNotification('Backup copied to clipboard!', 'success');
    } catch (error) {
        console.error('Failed to copy backup:', error);
        showNotification('Failed to copy backup', 'error');
    }
}

// Restore Wallet
async function restoreWallet() {
    const backupData = document.getElementById('restoreData').value;
    
    if (!backupData) {
        showNotification('Please paste backup data', 'error');
        return;
    }
    
    try {
        const response = await fetch(`${API_BASE}/wallet/restore`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                backup_data: backupData
            })
        });
        
        const result = await response.json();
        
        if (result.success) {
            showNotification('Wallet restored successfully', 'success');
            document.getElementById('restoreData').value = '';
            
            // Refresh all wallet data
            refreshBalances();
            loadWalletAddresses();
            refreshTransactionHistory();
            refreshKeys();
        } else {
            showNotification(result.message, 'error');
        }
    } catch (error) {
        console.error('Failed to restore wallet:', error);
        showNotification('Failed to restore wallet', 'error');
    }
} 