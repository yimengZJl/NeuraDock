use neuradock_infrastructure::http::UserInfo;

/// Check-in result for a single account
#[derive(Debug, Clone)]
pub struct AccountCheckInResult {
    pub account_name: String,
    pub success: bool,
    pub message: String,
    pub user_info: Option<UserInfo>,
}
