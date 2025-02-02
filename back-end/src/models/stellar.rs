use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Account {
    pub id: String,
    pub sequence: String,
    pub balances: Vec<Balance>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Balance {
    pub balance: String,
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Transaction {
    pub id: String,
    pub paging_token: String,
    pub successful: bool,
    pub hash: String,
    pub ledger: u32,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Ledger {
    pub id: String,
    pub paging_token: String,
    pub hash: String,
    pub prev_hash: String,
    pub sequence: u32,
    pub successful_transaction_count: u32,
    pub failed_transaction_count: u32,
    pub operation_count: u32,
    pub tx_set_operation_count: u32,
    pub closed_at: String,
    pub total_coins: String,
    pub fee_pool: String,
    pub base_fee_in_stroops: u32,
    pub base_reserve_in_stroops: u32,
    pub max_tx_set_size: u32,
    pub protocol_version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_xdr: Option<String>, // Opcional, caso não precise dele
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Offer {
    pub id: String,
    pub selling: Asset,
    pub buying: Asset,
    pub amount: String,
    pub price: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct Asset {
    pub asset_type: String,
    pub asset_code: Option<String>,
    pub asset_issuer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClaimableBalance {
    pub id: String,
    pub asset: String,
    pub amount: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct FeeStats {
    pub last_ledger_base_fee: String,
    pub last_ledger_capacity_usage: String,
}
