use chrono::NaiveDate;

pub struct StreakComputation {
    pub current_streak: u32,
    pub longest_streak: u32,
    pub total_check_in_days: u32,
    pub last_check_in_date: Option<NaiveDate>,
}
