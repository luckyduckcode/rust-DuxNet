// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[tauri::command]
async fn get_wallet_info() -> Result<serde_json::Value, String> {
    // This will be implemented to communicate with the DuxNet API
    Ok(serde_json::json!({
        "success": true,
        "message": "Wallet info from desktop app"
    }))
}

#[tauri::command]
async fn send_funds(to_address: String, amount: u64, currency: String) -> Result<serde_json::Value, String> {
    // This will be implemented to communicate with the DuxNet API
    Ok(serde_json::json!({
        "success": true,
        "transaction_id": "mock_tx_id",
        "message": format!("Sent {} {} to {}", amount, currency, to_address)
    }))
}

#[tauri::command]
async fn get_balances() -> Result<serde_json::Value, String> {
    // This will be implemented to communicate with the DuxNet API
    Ok(serde_json::json!({
        "success": true,
        "balances": {
            "BTC": "0.00000000",
            "ETH": "0.000000000000000000",
            "USDC": "1000.000000",
            "LTC": "0.00000000",
            "XMR": "0.000000000000",
            "DOGE": "10000.00000000"
        }
    }))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_wallet_info,
            send_funds,
            get_balances
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
} 