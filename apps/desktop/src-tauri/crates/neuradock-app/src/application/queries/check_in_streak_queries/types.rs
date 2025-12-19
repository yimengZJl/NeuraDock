use chrono::NaiveDate;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct DailySummaryRow {
    pub check_in_date: String,
    pub daily_total_income: f64,
    pub daily_balance: f64,
    pub daily_consumed: f64,
}

#[derive(FromRow)]
pub struct AccountInfoRow {
    pub name: String,
    pub provider_id: String,
    pub provider_name: String,
}

#[derive(FromRow)]
pub struct AccountListRow {
    pub account_id: String,
    pub name: String,
    pub provider_id: String,
    pub provider_name: String,
}

pub struct StreakComputation {
    pub current_streak: u32,
    pub longest_streak: u32,
    pub total_check_in_days: u32,
    pub last_check_in_date: Option<NaiveDate>,
}
