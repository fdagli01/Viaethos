//! Ethos Points rules (v1). See ARCHITECTURE.md §2 "Ethos Points rules".
//! Points are always written as signed events into `ethos_ledger` — never
//! as a stored running total. Totals are a SUM() view over the ledger.

const TICK_POINTS: i64 = 5;
const TASK_POINTS: i64 = 3;
const FOCUS_POINTS_PER_MINUTE: i64 = 1;
const INTENTION_SEALED_BONUS: i64 = 5;
const STREAK_CAP_DAYS: i64 = 30;
const STREAK_CAP_MULTIPLIER: f64 = 1.6;

/// Streak multiplier: 1 + min(streak_days, 30) * 0.02, capped at 1.6x.
pub fn streak_multiplier(streak_days: i64) -> f64 {
    let capped = streak_days.max(0).min(STREAK_CAP_DAYS) as f64;
    (1.0 + capped * 0.02).min(STREAK_CAP_MULTIPLIER)
}

fn apply_multiplier(base: i64, multiplier: f64) -> i64 {
    ((base as f64) * multiplier).round() as i64
}

/// Points for one Quick Tick completion, given the action's current streak.
pub fn tick_points(streak_days: i64) -> i64 {
    apply_multiplier(TICK_POINTS, streak_multiplier(streak_days))
}

/// Points for completing a one-off task. Flat — tasks don't recur, so no
/// streak concept applies.
pub fn task_points() -> i64 {
    TASK_POINTS
}

/// Points for a completed Focus Session, given elapsed minutes and streak.
/// Abandoned sessions earn 0 (still recorded — the Path stays honest).
pub fn focus_points(minutes: i64, streak_days: i64, had_intention_and_reflection: bool) -> i64 {
    let base = minutes.max(0) * FOCUS_POINTS_PER_MINUTE;
    let mut total = apply_multiplier(base, streak_multiplier(streak_days));
    if had_intention_and_reflection {
        total += INTENTION_SEALED_BONUS;
    }
    total
}
