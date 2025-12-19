use neuradock_infrastructure::http::UserInfo;

/// Check-in result for a single account
#[derive(Debug, Clone)]
pub struct AccountCheckInResult {
    pub account_id: String,
    pub account_name: String,
    pub success: bool,
    pub message: String,
    pub user_info: Option<UserInfo>,
}

/// Batch check-in result
#[derive(Debug, Clone)]
pub struct BatchCheckInResult {
    pub total: usize,
    pub success_count: usize,
    pub failed_count: usize,
    pub results: Vec<AccountCheckInResult>,
}
