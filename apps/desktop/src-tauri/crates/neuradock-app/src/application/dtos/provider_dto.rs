use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct ProviderDto {
    pub id: String,
    pub name: String,
    pub domain: String,
    pub is_builtin: bool,
    pub account_count: i32,
    pub supports_check_in: bool,
    pub check_in_bugged: bool,
    // API configuration fields
    pub login_path: String,
    pub sign_in_path: Option<String>,
    pub user_info_path: String,
    pub token_api_path: Option<String>,
    pub models_path: Option<String>,
    pub api_user_key: String,
    pub needs_waf_bypass: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct AddProviderInput {
    pub name: String,
    pub domain: String,
    pub login_path: String,
    pub sign_in_path: Option<String>,
    pub user_info_path: String,
    pub api_user_key: String,
    pub bypass_method: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
pub struct BrowserInfoDto {
    pub available: bool,
    pub path: Option<String>,
    pub message: Option<String>,
}
