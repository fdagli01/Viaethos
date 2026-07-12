use std::collections::{BTreeMap, BTreeSet};

use chrono::{Datelike, Duration, NaiveDate};

use super::models::Schedule;

/// Derive the current streak (in days, or weeks for `times_per_week`) for one
/// action. Never stored — recomputed from `entries` on demand.
///
/// `completed_counts` maps `occurred_on` -> number of completed entries that
/// day. A day "honors" the action once its count reaches `target_per_day`.
pub fn current_streak(
    schedule: &Schedule,
    target_per_day: i64,
    completed_counts: &BTreeMap<NaiveDate, i64>,
    today: NaiveDate,
) -> i64 {
    match schedule {
        Schedule::TimesPerWeek { n } => weekly_streak(*n, completed_counts, today),
        _ => daily_streak(schedule, target_per_day, completed_counts, today),
    }
}

fn daily_streak(
    schedule: &Schedule,
    target_per_day: i64,
    completed_counts: &BTreeMap<NaiveDate, i64>,
    today: NaiveDate,
) -> i64 {
    let mut streak = 0i64;
    let mut day = today;
    loop {
        let weekday = day.weekday().num_days_from_sunday();
        let due = schedule.is_due_on_weekday(weekday);
        let honored = completed_counts.get(&day).copied().unwrap_or(0) >= target_per_day;

        if due {
            if honored {
                streak += 1;
            } else if day == today {
                // Today isn't honored yet — doesn't break a streak still in progress.
            } else {
                break;
            }
        }
        // Not due: skip the day without affecting the streak.
        day -= Duration::days(1);

        // Safety bound: don't walk back forever on sparse data.
        if (today - day).num_days() > 3650 {
            break;
        }
    }
    streak
}

fn weekly_streak(n: u8, completed_counts: &BTreeMap<NaiveDate, i64>, today: NaiveDate) -> i64 {
    // Group completed counts into ISO weeks, then walk backward.
    let mut week_totals: BTreeMap<(i32, u32), i64> = BTreeMap::new();
    for (date, count) in completed_counts {
        let iso = date.iso_week();
        *week_totals.entry((iso.year(), iso.week())).or_insert(0) += count;
    }

    let mut streak = 0i64;
    let mut cursor = today;
    loop {
        let iso = cursor.iso_week();
        let key = (iso.year(), iso.week());
        let total = week_totals.get(&key).copied().unwrap_or(0);
        let is_current_week = key == (today.iso_week().year(), today.iso_week().week());

        if total >= n as i64 {
            streak += 1;
        } else if !is_current_week {
            break;
        }
        // Move to the previous week (any day in it).
        cursor -= Duration::weeks(1);

        if (today - cursor).num_weeks() > 520 {
            break;
        }
    }
    streak
}

/// Best (longest) streak ever observed for a daily/weekdays/days schedule,
/// scanning forward across the full history of completed days.
pub fn best_streak(
    schedule: &Schedule,
    target_per_day: i64,
    completed_counts: &BTreeMap<NaiveDate, i64>,
) -> i64 {
    if completed_counts.is_empty() {
        return 0;
    }
    let all_days: BTreeSet<NaiveDate> = completed_counts.keys().copied().collect();
    let first = *all_days.iter().next().unwrap();
    let last = *all_days.iter().last().unwrap();

    let mut best = 0i64;
    let mut running = 0i64;
    let mut day = first;
    while day <= last {
        let weekday = day.weekday().num_days_from_sunday();
        let due = schedule.is_due_on_weekday(weekday);
        let honored = completed_counts.get(&day).copied().unwrap_or(0) >= target_per_day;
        if due {
            if honored {
                running += 1;
                best = best.max(running);
            } else {
                running = 0;
            }
        }
        day += Duration::days(1);
    }
    best
}
