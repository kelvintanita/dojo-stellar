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
    pub sequence: u32,
    pub hash: String,
    pub transaction_count: u32,
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
