use sqlx::SqlitePool;
use std::sync::Arc;

use crate::application::dtos::{
    CheckInCalendarDto, CheckInDayDto, CheckInStreakDto, CheckInTrendDto,
};
use neuradock_domain::shared::DomainError;

mod calendar;
mod helpers;
mod streak;
mod trend;
mod types;

pub struct CheckInStreakQueries {
    db: Arc<SqlitePool>,
}

impl CheckInStreakQueries {
    pub fn new(db: Arc<SqlitePool>) -> Self {
        Self { db }
    }

    /// Get streak statistics for a single account
    pub async fn get_streak_stats(
        &self,
        account_id: &str,
    ) -> Result<CheckInStreakDto, DomainError> {
        streak::get_streak_stats(&self.db, account_id).await
    }

    /// Get streak statistics for all accounts
    pub async fn get_all_streaks(&self) -> Result<Vec<CheckInStreakDto>, DomainError> {
        streak::get_all_streaks(&self.db).await
    }

    /// Get check-in calendar for a specific month
    pub async fn get_calendar(
        &self,
        account_id: &str,
        year: i32,
        month: u32,
    ) -> Result<CheckInCalendarDto, DomainError> {
        calendar::get_calendar(&self.db, account_id, year, month).await
    }

    /// Get check-in trend data (last N days)
    pub async fn get_trend(
        &self,
        account_id: &str,
        days: u32,
    ) -> Result<CheckInTrendDto, DomainError> {
        trend::get_trend(&self.db, account_id, days).await
    }

    /// Get details for a specific day
    pub async fn get_day_detail(
        &self,
        account_id: &str,
        date: &str,
    ) -> Result<CheckInDayDto, DomainError> {
        trend::get_day_detail(&self.db, account_id, date).await
    }

    /// Recalculate all streaks from balance_history
    pub async fn recalculate_all_streaks(&self) -> Result<(), DomainError> {
        streak::recalculate_all_streaks(&self.db).await
    }
}
