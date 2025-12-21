use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::shared::{AccountId, DomainError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceHistoryRecord {
    id: String,
    account_id: AccountId,
    current_balance: f64,
    total_consumed: f64,
    total_income: f64,
    recorded_at: DateTime<Utc>,
}

impl BalanceHistoryRecord {
    pub fn new(
        id: String,
        account_id: AccountId,
        current_balance: f64,
        total_consumed: f64,
        total_income: f64,
        recorded_at: DateTime<Utc>,
    ) -> Result<Self, DomainError> {
        if id.is_empty() {
            return Err(DomainError::Validation(
                "Balance history id cannot be empty".to_string(),
            ));
        }
        if current_balance < 0.0 {
            return Err(DomainError::Validation(
                "Current balance cannot be negative".to_string(),
            ));
        }
        if total_consumed < 0.0 {
            return Err(DomainError::Validation(
                "Total consumed cannot be negative".to_string(),
            ));
        }
        if total_income < 0.0 {
            return Err(DomainError::Validation(
                "Total income cannot be negative".to_string(),
            ));
        }

        Ok(Self {
            id,
            account_id,
            current_balance,
            total_consumed,
            total_income,
            recorded_at,
        })
    }

    pub fn restore(
        id: String,
        account_id: AccountId,
        current_balance: f64,
        total_consumed: f64,
        total_income: f64,
        recorded_at: DateTime<Utc>,
    ) -> Self {
        Self {
            id,
            account_id,
            current_balance,
            total_consumed,
            total_income,
            recorded_at,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn account_id(&self) -> &AccountId {
        &self.account_id
    }

    pub fn current_balance(&self) -> f64 {
        self.current_balance
    }

    pub fn total_consumed(&self) -> f64 {
        self.total_consumed
    }

    pub fn total_income(&self) -> f64 {
        self.total_income
    }

    pub fn recorded_at(&self) -> DateTime<Utc> {
        self.recorded_at
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceHistoryDailySummary {
    check_in_date: NaiveDate,
    daily_total_income: f64,
    daily_balance: f64,
    daily_consumed: f64,
}

impl BalanceHistoryDailySummary {
    pub fn new(
        check_in_date: NaiveDate,
        daily_total_income: f64,
        daily_balance: f64,
        daily_consumed: f64,
    ) -> Result<Self, DomainError> {
        if daily_total_income < 0.0 {
            return Err(DomainError::Validation(
                "Daily total income cannot be negative".to_string(),
            ));
        }
        if daily_balance < 0.0 {
            return Err(DomainError::Validation(
                "Daily balance cannot be negative".to_string(),
            ));
        }
        if daily_consumed < 0.0 {
            return Err(DomainError::Validation(
                "Daily consumed cannot be negative".to_string(),
            ));
        }

        Ok(Self {
            check_in_date,
            daily_total_income,
            daily_balance,
            daily_consumed,
        })
    }

    pub fn restore(
        check_in_date: NaiveDate,
        daily_total_income: f64,
        daily_balance: f64,
        daily_consumed: f64,
    ) -> Self {
        Self {
            check_in_date,
            daily_total_income,
            daily_balance,
            daily_consumed,
        }
    }

    pub fn check_in_date(&self) -> NaiveDate {
        self.check_in_date
    }

    pub fn daily_total_income(&self) -> f64 {
        self.daily_total_income
    }

    pub fn daily_balance(&self) -> f64 {
        self.daily_balance
    }

    pub fn daily_consumed(&self) -> f64 {
        self.daily_consumed
    }
}
