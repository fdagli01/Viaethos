use chrono::{Datelike, NaiveDate};

use super::models::Schedule;

/// Answers "what does today ask?" for one action.
///
/// `entries_this_week` is the count of completed entries for the action
/// since the most recent Monday (inclusive) — only meaningful for
/// `TimesPerWeek`, ignored otherwise.
pub fn is_due_on(schedule: &Schedule, date: NaiveDate, entries_this_week: u32) -> bool {
    match schedule {
        Schedule::TimesPerWeek { n } => entries_this_week < *n as u32,
        other => other.is_due_on_weekday(date.weekday().num_days_from_sunday()),
    }
}
