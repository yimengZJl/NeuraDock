use serde::{Deserialize, Serialize};
use specta::Type;

use neuradock_domain::check_in::Balance;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct BalanceDto {
    pub current_balance: f64,
    pub total_consumed: f64,
    pub total_income: f64,
}

impl From<Balance> for BalanceDto {
    fn from(b: Balance) -> Self {
        Self {
            current_balance: b.current_balance,
            total_consumed: b.total_consumed,
            total_income: b.total_income,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProviderBalanceDto {
    pub provider_id: String,
    pub provider_name: String,
    pub current_balance: f64,
    pub total_consumed: f64,
    pub total_income: f64,
    pub account_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct BalanceStatisticsDto {
    pub providers: Vec<ProviderBalanceDto>,
    pub total_current_balance: f64,
    pub total_consumed: f64,
    pub total_income: f64,
}
