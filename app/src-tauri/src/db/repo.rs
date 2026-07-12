use std::collections::BTreeMap;

use chrono::{Datelike, Duration, Local, NaiveDate};
use rusqlite::{params, Connection, OptionalExtension, Result};
use uuid::Uuid;

use crate::domain::models::{
    Action, ActionKind, Course, Entry, FoodItem, Lesson, Meal, Pillar, Schedule, SleepLog, Task,
};

/// "Today" honors the user's day-boundary setting (default midnight) — a
/// night owl can set it to 04:00 so a 2am session still lands on
/// yesterday's row rather than fracturing the day artificially.
pub fn today(conn: &Connection) -> NaiveDate {
    let boundary_hour: i64 = get_setting(conn, "day_boundary_hour")
        .ok()
        .flatten()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    if boundary_hour == 0 {
        return Local::now().date_naive();
    }
    (Local::now() - Duration::hours(boundary_hour)).date_naive()
}

pub fn now_ts() -> i64 {
    chrono::Utc::now().timestamp()
}

// ---------------------------------------------------------------- seeding --

pub fn seed_if_empty(conn: &Connection) -> Result<()> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM pillars", [], |r| r.get(0))?;
    if count > 0 {
        return Ok(());
    }

    let pillars = [
        ("Mind", "#3b7fd9"),
        ("Body", "#d94f38"),
        ("Craft", "#b8811f"),
        ("Life", "#2a9a62"),
    ];
    let mut pillar_ids: Vec<String> = Vec::new();
    for (i, (name, color)) in pillars.iter().enumerate() {
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO pillars (id, name, color_token, icon, sort_order, archived_at)
             VALUES (?1, ?2, ?3, NULL, ?4, NULL)",
            params![id, name, color, i as i64],
        )?;
        pillar_ids.push(id);
    }

    let now = now_ts();
    let daily = Schedule::Daily.to_json();
    let starters: [(usize, &str, ActionKind, Option<i64>, i64); 8] = [
        (0, "Deep Focus Session", ActionKind::Focus, Some(25), 1),
        (0, "Read 10 Pages", ActionKind::Tick, None, 1),
        (1, "Movement Session", ActionKind::Focus, Some(30), 1),
        (1, "Drink Water", ActionKind::Tick, None, 8),
        (2, "Craft Session", ActionKind::Focus, Some(45), 1),
        (2, "Sketch Practice", ActionKind::Tick, None, 1),
        (3, "Life Admin Session", ActionKind::Focus, Some(20), 1),
        (3, "Take Vitamins", ActionKind::Tick, None, 1),
    ];
    for (i, (pillar_idx, name, kind, minutes, target)) in starters.iter().enumerate() {
        let id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO actions
                (id, pillar_id, name, kind, default_minutes, schedule, target_per_day, sort_order, created_at, archived_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, NULL)",
            params![
                id,
                pillar_ids[*pillar_idx],
                name,
                kind.as_str(),
                minutes,
                daily,
                target,
                i as i64,
                now
            ],
        )?;
    }
    Ok(())
}

/// A modest packaged food list — enough to search meaningfully out of the
/// box. Users can add their own via `add_food_item`.
pub fn seed_food_items_if_empty(conn: &Connection) -> Result<()> {
    let count: i64 = conn.query_row("SELECT COUNT(*) FROM food_items", [], |r| r.get(0))?;
    if count > 0 {
        return Ok(());
    }
    // (name, kcal/100g, protein/100g, carb/100g, fat/100g)
    let foods: &[(&str, f64, f64, f64, f64)] = &[
        ("Chicken breast, cooked", 165.0, 31.0, 0.0, 3.6),
        ("Egg, whole", 155.0, 13.0, 1.1, 11.0),
        ("White rice, cooked", 130.0, 2.7, 28.0, 0.3),
        ("Brown rice, cooked", 123.0, 2.6, 26.0, 1.0),
        ("Oats, dry", 389.0, 16.9, 66.0, 6.9),
        ("Whole milk", 61.0, 3.2, 4.8, 3.3),
        ("Greek yogurt, plain", 59.0, 10.0, 3.6, 0.4),
        ("Banana", 89.0, 1.1, 23.0, 0.3),
        ("Apple", 52.0, 0.3, 14.0, 0.2),
        ("Almonds", 579.0, 21.0, 22.0, 50.0),
        ("Olive oil", 884.0, 0.0, 0.0, 100.0),
        ("Whole wheat bread", 247.0, 13.0, 41.0, 4.2),
        ("White bread", 265.0, 9.0, 49.0, 3.2),
        ("Cheddar cheese", 402.0, 25.0, 1.3, 33.0),
        ("Salmon, cooked", 208.0, 20.0, 0.0, 13.0),
        ("Ground beef, cooked", 250.0, 26.0, 0.0, 15.0),
        ("Lentils, cooked", 116.0, 9.0, 20.0, 0.4),
        ("Chickpeas, cooked", 164.0, 8.9, 27.0, 2.6),
        ("Broccoli, cooked", 35.0, 2.4, 7.2, 0.4),
        ("Potato, boiled", 87.0, 1.9, 20.0, 0.1),
        ("Sweet potato, baked", 90.0, 2.0, 21.0, 0.2),
        ("Avocado", 160.0, 2.0, 8.5, 15.0),
        ("Peanut butter", 588.0, 25.0, 20.0, 50.0),
        ("Dark chocolate", 546.0, 4.9, 61.0, 31.0),
        ("Turkish tea (unsweetened)", 1.0, 0.0, 0.2, 0.0),
        ("Simit", 275.0, 8.5, 51.0, 4.5),
        ("Lentil soup (mercimek çorbası)", 90.0, 5.0, 13.0, 2.0),
        ("Ayran", 34.0, 1.6, 2.0, 1.9),
    ];
    for (name, kcal, protein, carb, fat) in foods {
        conn.execute(
            "INSERT INTO food_items (id, name, kcal_per_100g, protein_per_100g, carb_per_100g, fat_per_100g, user_defined)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, 0)",
            params![Uuid::new_v4().to_string(), name, kcal, protein, carb, fat],
        )?;
    }
    Ok(())
}

// --------------------------------------------------------------- pillars --

pub fn list_pillars(conn: &Connection) -> Result<Vec<Pillar>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, color_token, icon, sort_order FROM pillars
         WHERE archived_at IS NULL ORDER BY sort_order",
    )?;
    let rows = stmt.query_map([], |r| {
        Ok(Pillar {
            id: r.get(0)?,
            name: r.get(1)?,
            color_token: r.get(2)?,
            icon: r.get(3)?,
            sort_order: r.get(4)?,
        })
    })?;
    rows.collect()
}

pub fn update_pillar(conn: &Connection, pillar_id: &str, name: &str, color_token: &str) -> Result<()> {
    conn.execute(
        "UPDATE pillars SET name = ?1, color_token = ?2 WHERE id = ?3",
        params![name, color_token, pillar_id],
    )?;
    Ok(())
}

// --------------------------------------------------------------- actions --

fn row_to_action(r: &rusqlite::Row) -> rusqlite::Result<Action> {
    let kind_str: String = r.get(3)?;
    let schedule_str: String = r.get(5)?;
    Ok(Action {
        id: r.get(0)?,
        pillar_id: r.get(1)?,
        name: r.get(2)?,
        kind: ActionKind::from_str(&kind_str),
        default_minutes: r.get(4)?,
        schedule: Schedule::from_json(&schedule_str),
        target_per_day: r.get(6)?,
        sort_order: r.get(7)?,
    })
}

pub fn list_actions(conn: &Connection) -> Result<Vec<Action>> {
    let mut stmt = conn.prepare(
        "SELECT id, pillar_id, name, kind, default_minutes, schedule, target_per_day, sort_order
         FROM actions WHERE archived_at IS NULL ORDER BY sort_order",
    )?;
    let rows = stmt.query_map([], row_to_action)?;
    rows.collect()
}

pub fn get_action(conn: &Connection, action_id: &str) -> Result<Action> {
    conn.query_row(
        "SELECT id, pillar_id, name, kind, default_minutes, schedule, target_per_day, sort_order
         FROM actions WHERE id = ?1",
        params![action_id],
        row_to_action,
    )
}

/// All actions regardless of archived state, each paired with whether it's
/// archived — Manage needs to show and un-archive retired actions.
pub fn list_all_actions(conn: &Connection) -> Result<Vec<(Action, bool)>> {
    let mut stmt = conn.prepare(
        "SELECT id, pillar_id, name, kind, default_minutes, schedule, target_per_day, sort_order, archived_at
         FROM actions ORDER BY sort_order",
    )?;
    let rows = stmt.query_map([], |r| {
        let action = row_to_action(r)?;
        let archived_at: Option<i64> = r.get(8)?;
        Ok((action, archived_at.is_some()))
    })?;
    rows.collect()
}

#[allow(clippy::too_many_arguments)]
pub fn add_action(
    conn: &Connection,
    pillar_id: &str,
    name: &str,
    kind: ActionKind,
    default_minutes: Option<i64>,
    schedule: &Schedule,
    target_per_day: i64,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO actions
            (id, pillar_id, name, kind, default_minutes, schedule, target_per_day, sort_order, created_at, archived_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8, NULL)",
        params![
            id,
            pillar_id,
            name,
            kind.as_str(),
            default_minutes,
            schedule.to_json(),
            target_per_day,
            now_ts()
        ],
    )?;
    Ok(id)
}

#[allow(clippy::too_many_arguments)]
pub fn update_action(
    conn: &Connection,
    action_id: &str,
    name: &str,
    default_minutes: Option<i64>,
    schedule: &Schedule,
    target_per_day: i64,
) -> Result<()> {
    conn.execute(
        "UPDATE actions SET name = ?1, default_minutes = ?2, schedule = ?3, target_per_day = ?4
         WHERE id = ?5",
        params![name, default_minutes, schedule.to_json(), target_per_day, action_id],
    )?;
    Ok(())
}

pub fn set_action_archived(conn: &Connection, action_id: &str, archived: bool) -> Result<()> {
    let value = if archived { Some(now_ts()) } else { None };
    conn.execute(
        "UPDATE actions SET archived_at = ?1 WHERE id = ?2",
        params![value, action_id],
    )?;
    Ok(())
}

// --------------------------------------------------------------- entries --

/// Completed-entry counts per day for one action, since `since` (inclusive).
/// A day "honors" the action once its count reaches the action's target.
pub fn completed_counts_since(
    conn: &Connection,
    action_id: &str,
    since: NaiveDate,
) -> Result<BTreeMap<NaiveDate, i64>> {
    let mut stmt = conn.prepare(
        "SELECT occurred_on, COUNT(*) FROM entries
         WHERE action_id = ?1 AND outcome = 'completed' AND occurred_on >= ?2
         GROUP BY occurred_on",
    )?;
    let rows = stmt.query_map(params![action_id, since.to_string()], |r| {
        let date_str: String = r.get(0)?;
        let count: i64 = r.get(1)?;
        Ok((date_str, count))
    })?;
    let mut map = BTreeMap::new();
    for row in rows {
        let (date_str, count) = row?;
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            map.insert(date, count);
        }
    }
    Ok(map)
}

pub fn active_focus_entry(conn: &Connection, action_id: &str) -> Result<Option<Entry>> {
    conn.query_row(
        "SELECT id, action_id, kind, occurred_on, intention, planned_minutes,
                started_at, ended_at, outcome, reflection, lesson_id, created_at
         FROM entries
         WHERE action_id = ?1 AND kind = 'focus' AND ended_at IS NULL
         ORDER BY created_at DESC LIMIT 1",
        params![action_id],
        row_to_entry,
    )
    .optional()
}

fn row_to_entry(r: &rusqlite::Row) -> rusqlite::Result<Entry> {
    let kind_str: String = r.get(2)?;
    Ok(Entry {
        id: r.get(0)?,
        action_id: r.get(1)?,
        kind: ActionKind::from_str(&kind_str),
        occurred_on: r.get(3)?,
        intention: r.get(4)?,
        planned_minutes: r.get(5)?,
        started_at: r.get(6)?,
        ended_at: r.get(7)?,
        outcome: r.get(8)?,
        reflection: r.get(9)?,
        lesson_id: r.get(10)?,
        created_at: r.get(11)?,
    })
}

pub fn get_entry(conn: &Connection, entry_id: &str) -> Result<Entry> {
    conn.query_row(
        "SELECT id, action_id, kind, occurred_on, intention, planned_minutes,
                started_at, ended_at, outcome, reflection, lesson_id, created_at
         FROM entries WHERE id = ?1",
        params![entry_id],
        row_to_entry,
    )
}

pub fn insert_tick(conn: &Connection, action_id: &str) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let now = now_ts();
    conn.execute(
        "INSERT INTO entries
            (id, action_id, kind, occurred_on, intention, planned_minutes,
             started_at, ended_at, outcome, reflection, lesson_id, created_at)
         VALUES (?1, ?2, 'tick', ?3, NULL, NULL, ?4, ?4, 'completed', NULL, NULL, ?4)",
        params![id, action_id, today(conn).to_string(), now],
    )?;
    Ok(id)
}

pub fn start_focus(
    conn: &Connection,
    action_id: &str,
    intention: Option<&str>,
    planned_minutes: i64,
    lesson_id: Option<&str>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let now = now_ts();
    conn.execute(
        "INSERT INTO entries
            (id, action_id, kind, occurred_on, intention, planned_minutes,
             started_at, ended_at, outcome, reflection, lesson_id, created_at)
         VALUES (?1, ?2, 'focus', ?3, ?4, ?5, ?6, NULL, NULL, NULL, ?7, ?6)",
        params![id, action_id, today(conn).to_string(), intention, planned_minutes, now, lesson_id],
    )?;
    Ok(id)
}

pub fn end_focus(
    conn: &Connection,
    entry_id: &str,
    outcome: &str,
    reflection: Option<&str>,
) -> Result<Entry> {
    let now = now_ts();
    conn.execute(
        "UPDATE entries SET ended_at = ?1, outcome = ?2, reflection = ?3 WHERE id = ?4",
        params![now, outcome, reflection, entry_id],
    )?;
    get_entry(conn, entry_id)
}

pub fn insert_ledger(
    conn: &Connection,
    pillar_id: &str,
    action_id: Option<&str>,
    entry_id: Option<&str>,
    points: i64,
    reason: &str,
) -> Result<()> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO ethos_ledger (id, pillar_id, action_id, entry_id, points, reason, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![id, pillar_id, action_id, entry_id, points, reason, now_ts()],
    )?;
    Ok(())
}

// -------------------------------------------------------------- Sofra --

fn row_to_food_item(r: &rusqlite::Row) -> rusqlite::Result<FoodItem> {
    let user_defined: i64 = r.get(6)?;
    Ok(FoodItem {
        id: r.get(0)?,
        name: r.get(1)?,
        kcal_per_100g: r.get(2)?,
        protein_per_100g: r.get(3)?,
        carb_per_100g: r.get(4)?,
        fat_per_100g: r.get(5)?,
        user_defined: user_defined != 0,
    })
}

pub fn search_food_items(conn: &Connection, query: &str) -> Result<Vec<FoodItem>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, kcal_per_100g, protein_per_100g, carb_per_100g, fat_per_100g, user_defined
         FROM food_items WHERE name LIKE ?1 ORDER BY name LIMIT 25",
    )?;
    let pattern = format!("%{query}%");
    let rows = stmt.query_map(params![pattern], row_to_food_item)?;
    rows.collect()
}

pub fn add_food_item(
    conn: &Connection,
    name: &str,
    kcal_per_100g: f64,
    protein_per_100g: f64,
    carb_per_100g: f64,
    fat_per_100g: f64,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO food_items (id, name, kcal_per_100g, protein_per_100g, carb_per_100g, fat_per_100g, user_defined)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1)",
        params![id, name, kcal_per_100g, protein_per_100g, carb_per_100g, fat_per_100g],
    )?;
    Ok(id)
}

fn row_to_meal(r: &rusqlite::Row) -> rusqlite::Result<Meal> {
    Ok(Meal {
        id: r.get(0)?,
        occurred_on: r.get(1)?,
        time_slot: r.get(2)?,
        name: r.get(3)?,
        kcal: r.get(4)?,
        protein_g: r.get(5)?,
        carb_g: r.get(6)?,
        fat_g: r.get(7)?,
        note: r.get(8)?,
        created_at: r.get(9)?,
    })
}

pub fn list_meals_on(conn: &Connection, date: NaiveDate) -> Result<Vec<Meal>> {
    let mut stmt = conn.prepare(
        "SELECT id, occurred_on, time_slot, name, kcal, protein_g, carb_g, fat_g, note, created_at
         FROM meals WHERE occurred_on = ?1 ORDER BY created_at",
    )?;
    let rows = stmt.query_map(params![date.to_string()], row_to_meal)?;
    rows.collect()
}

#[allow(clippy::too_many_arguments)]
pub fn add_meal(
    conn: &Connection,
    time_slot: &str,
    name: &str,
    kcal: f64,
    protein_g: f64,
    carb_g: f64,
    fat_g: f64,
    note: Option<&str>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO meals (id, occurred_on, time_slot, name, kcal, protein_g, carb_g, fat_g, note, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![id, today(conn).to_string(), time_slot, name, kcal, protein_g, carb_g, fat_g, note, now_ts()],
    )?;
    Ok(id)
}

pub fn delete_meal(conn: &Connection, meal_id: &str) -> Result<()> {
    conn.execute("DELETE FROM meals WHERE id = ?1", params![meal_id])?;
    Ok(())
}

pub fn kcal_today(conn: &Connection) -> Result<f64> {
    conn.query_row(
        "SELECT COALESCE(SUM(kcal), 0) FROM meals WHERE occurred_on = ?1",
        params![today(conn).to_string()],
        |r| r.get(0),
    )
}

// ------------------------------------------------------------------ Uyku --

fn row_to_sleep_log(r: &rusqlite::Row) -> rusqlite::Result<SleepLog> {
    Ok(SleepLog {
        id: r.get(0)?,
        date: r.get(1)?,
        bed_at: r.get(2)?,
        woke_at: r.get(3)?,
        quality_1_5: r.get(4)?,
        created_at: r.get(5)?,
    })
}

pub fn log_sleep(
    conn: &Connection,
    date: &str,
    bed_at: &str,
    woke_at: &str,
    quality_1_5: i64,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO sleep_logs (id, date, bed_at, woke_at, quality_1_5, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(date) DO UPDATE SET
            bed_at = excluded.bed_at, woke_at = excluded.woke_at,
            quality_1_5 = excluded.quality_1_5",
        params![id, date, bed_at, woke_at, quality_1_5, now_ts()],
    )?;
    Ok(id)
}

pub fn get_sleep_on(conn: &Connection, date: NaiveDate) -> Result<Option<SleepLog>> {
    conn.query_row(
        "SELECT id, date, bed_at, woke_at, quality_1_5, created_at FROM sleep_logs WHERE date = ?1",
        params![date.to_string()],
        row_to_sleep_log,
    )
    .optional()
}

pub fn sleep_by_day(conn: &Connection, since: NaiveDate) -> Result<Vec<SleepLog>> {
    let mut stmt = conn.prepare(
        "SELECT id, date, bed_at, woke_at, quality_1_5, created_at
         FROM sleep_logs WHERE date >= ?1 ORDER BY date",
    )?;
    let rows = stmt.query_map(params![since.to_string()], row_to_sleep_log)?;
    rows.collect()
}

// ------------------------------------------------------------- Müfredat --

fn row_to_course(r: &rusqlite::Row) -> rusqlite::Result<Course> {
    Ok(Course {
        id: r.get(0)?,
        name: r.get(1)?,
        pillar_id: r.get(2)?,
        action_id: r.get(3)?,
        color_token: r.get(4)?,
        target_hours_week: r.get(5)?,
    })
}

pub fn list_courses(conn: &Connection) -> Result<Vec<Course>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, pillar_id, action_id, color_token, target_hours_week
         FROM courses WHERE archived_at IS NULL ORDER BY created_at",
    )?;
    let rows = stmt.query_map([], row_to_course)?;
    rows.collect()
}

pub fn get_course(conn: &Connection, course_id: &str) -> Result<Course> {
    conn.query_row(
        "SELECT id, name, pillar_id, action_id, color_token, target_hours_week
         FROM courses WHERE id = ?1",
        params![course_id],
        row_to_course,
    )
}

/// A course carries its own Focus action, so studying it flows through the
/// exact same Ritual/Mosaic/streak machinery as any other Focus Session.
pub fn add_course(
    conn: &Connection,
    name: &str,
    pillar_id: &str,
    target_hours_week: f64,
) -> Result<String> {
    let color_token: String = conn.query_row(
        "SELECT color_token FROM pillars WHERE id = ?1",
        params![pillar_id],
        |r| r.get(0),
    )?;
    let action_id = Uuid::new_v4().to_string();
    let now = now_ts();
    conn.execute(
        "INSERT INTO actions
            (id, pillar_id, name, kind, default_minutes, schedule, target_per_day, sort_order, created_at, archived_at)
         VALUES (?1, ?2, ?3, 'focus', 30, ?4, 1, 0, ?5, NULL)",
        params![action_id, pillar_id, format!("Study: {name}"), Schedule::Daily.to_json(), now],
    )?;
    let course_id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO courses (id, name, pillar_id, action_id, color_token, target_hours_week, created_at, archived_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, NULL)",
        params![course_id, name, pillar_id, action_id, color_token, target_hours_week, now],
    )?;
    Ok(course_id)
}

fn row_to_lesson(r: &rusqlite::Row) -> rusqlite::Result<Lesson> {
    Ok(Lesson {
        id: r.get(0)?,
        course_id: r.get(1)?,
        title: r.get(2)?,
        planned_on: r.get(3)?,
        status: r.get(4)?,
        review_of: r.get(5)?,
        created_at: r.get(6)?,
    })
}

pub fn add_lesson(conn: &Connection, course_id: &str, title: &str, planned_on: &str) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO lessons (id, course_id, title, planned_on, status, review_of, created_at)
         VALUES (?1, ?2, ?3, ?4, 'planned', NULL, ?5)",
        params![id, course_id, title, planned_on, now_ts()],
    )?;
    Ok(id)
}

/// Lessons due today or overdue (still planned) for one course.
pub fn lessons_due(conn: &Connection, course_id: &str) -> Result<Vec<Lesson>> {
    let mut stmt = conn.prepare(
        "SELECT id, course_id, title, planned_on, status, review_of, created_at
         FROM lessons WHERE course_id = ?1 AND status = 'planned' AND planned_on <= ?2
         ORDER BY planned_on",
    )?;
    let rows = stmt.query_map(params![course_id, today(conn).to_string()], row_to_lesson)?;
    rows.collect()
}

pub fn get_lesson(conn: &Connection, lesson_id: &str) -> Result<Lesson> {
    conn.query_row(
        "SELECT id, course_id, title, planned_on, status, review_of, created_at
         FROM lessons WHERE id = ?1",
        params![lesson_id],
        row_to_lesson,
    )
}

/// (done, total) lesson counts for one course — the field row's fill level.
pub fn course_progress(conn: &Connection, course_id: &str) -> Result<(i64, i64)> {
    let total: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lessons WHERE course_id = ?1",
        params![course_id],
        |r| r.get(0),
    )?;
    let done: i64 = conn.query_row(
        "SELECT COUNT(*) FROM lessons WHERE course_id = ?1 AND status = 'done'",
        params![course_id],
        |r| r.get(0),
    )?;
    Ok((done, total))
}

/// How many prior reviews already precede this lesson in its review_of chain
/// — 0 for an original topic, 1 for its first review, and so on.
fn review_depth(conn: &Connection, lesson_id: &str) -> Result<u32> {
    let mut depth = 0u32;
    let mut current = lesson_id.to_string();
    loop {
        let parent: Option<String> = conn
            .query_row(
                "SELECT review_of FROM lessons WHERE id = ?1",
                params![current],
                |r| r.get(0),
            )
            .optional()?
            .flatten();
        match parent {
            Some(p) => {
                depth += 1;
                current = p;
                if depth > 20 {
                    break; // safety bound
                }
            }
            None => break,
        }
    }
    Ok(depth)
}

const SPACED_REPETITION_INTERVALS_DAYS: [i64; 5] = [1, 3, 7, 16, 35];

/// Marks a lesson done and schedules its next spaced-repetition review.
/// Returns the newly scheduled review lesson's id.
pub fn complete_lesson(conn: &Connection, lesson_id: &str) -> Result<String> {
    conn.execute(
        "UPDATE lessons SET status = 'done' WHERE id = ?1",
        params![lesson_id],
    )?;
    let lesson = get_lesson(conn, lesson_id)?;
    let depth = review_depth(conn, lesson_id)?;
    let interval = SPACED_REPETITION_INTERVALS_DAYS
        [(depth as usize).min(SPACED_REPETITION_INTERVALS_DAYS.len() - 1)];
    let next_on = today(conn) + Duration::days(interval);
    add_lesson_review(conn, &lesson.course_id, &lesson.title, &next_on.to_string(), lesson_id)
}

fn add_lesson_review(
    conn: &Connection,
    course_id: &str,
    title: &str,
    planned_on: &str,
    review_of: &str,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO lessons (id, course_id, title, planned_on, status, review_of, created_at)
         VALUES (?1, ?2, ?3, ?4, 'planned', ?5, ?6)",
        params![id, course_id, title, planned_on, review_of, now_ts()],
    )?;
    Ok(id)
}

pub fn skip_lesson(conn: &Connection, lesson_id: &str) -> Result<()> {
    conn.execute(
        "UPDATE lessons SET status = 'skipped' WHERE id = ?1",
        params![lesson_id],
    )?;
    Ok(())
}

// ----------------------------------------------------------------- tasks --

fn row_to_task(r: &rusqlite::Row) -> rusqlite::Result<Task> {
    Ok(Task {
        id: r.get(0)?,
        pillar_id: r.get(1)?,
        action_id: r.get(2)?,
        title: r.get(3)?,
        due_on: r.get(4)?,
        completed_at: r.get(5)?,
        created_at: r.get(6)?,
    })
}

/// Open tasks (not completed), due today or overdue first, then undated,
/// then future-dated — the order they should read on the Path.
pub fn list_open_tasks(conn: &Connection) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare(
        "SELECT id, pillar_id, action_id, title, due_on, completed_at, created_at
         FROM tasks WHERE completed_at IS NULL
         ORDER BY CASE WHEN due_on IS NULL THEN 1 ELSE 0 END, due_on, created_at",
    )?;
    let rows = stmt.query_map([], row_to_task)?;
    rows.collect()
}

pub fn add_task(
    conn: &Connection,
    title: &str,
    pillar_id: Option<&str>,
    due_on: Option<&str>,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    conn.execute(
        "INSERT INTO tasks (id, pillar_id, action_id, title, due_on, completed_at, created_at)
         VALUES (?1, ?2, NULL, ?3, ?4, NULL, ?5)",
        params![id, pillar_id, title, due_on, now_ts()],
    )?;
    Ok(id)
}

pub fn get_task(conn: &Connection, task_id: &str) -> Result<Task> {
    conn.query_row(
        "SELECT id, pillar_id, action_id, title, due_on, completed_at, created_at
         FROM tasks WHERE id = ?1",
        params![task_id],
        row_to_task,
    )
}

pub fn complete_task(conn: &Connection, task_id: &str) -> Result<Task> {
    conn.execute(
        "UPDATE tasks SET completed_at = ?1 WHERE id = ?2",
        params![now_ts(), task_id],
    )?;
    get_task(conn, task_id)
}

pub fn delete_task(conn: &Connection, task_id: &str) -> Result<()> {
    conn.execute("DELETE FROM tasks WHERE id = ?1", params![task_id])?;
    Ok(())
}

// ------------------------------------------------------------- settings --

pub fn get_setting(conn: &Connection, key: &str) -> Result<Option<String>> {
    conn.query_row(
        "SELECT value FROM settings WHERE key = ?1",
        params![key],
        |r| r.get(0),
    )
    .optional()
}

pub fn set_setting(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO settings (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![key, value],
    )?;
    Ok(())
}

/// Distinct pillar ids honored (at least one completed entry) on each day,
/// since `since` (inclusive) — feeds the Path ribbon's per-day color mix.
pub fn pillar_mix_by_day(
    conn: &Connection,
    since: NaiveDate,
) -> Result<BTreeMap<NaiveDate, Vec<String>>> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT e.occurred_on, a.pillar_id
         FROM entries e
         JOIN actions a ON a.id = e.action_id
         WHERE e.outcome = 'completed' AND e.occurred_on >= ?1
         ORDER BY e.occurred_on",
    )?;
    let rows = stmt.query_map(params![since.to_string()], |r| {
        let date_str: String = r.get(0)?;
        let pillar_id: String = r.get(1)?;
        Ok((date_str, pillar_id))
    })?;
    let mut map: BTreeMap<NaiveDate, Vec<String>> = BTreeMap::new();
    for row in rows {
        let (date_str, pillar_id) = row?;
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            map.entry(date).or_default().push(pillar_id);
        }
    }
    Ok(map)
}

/// Total minutes spent in completed focus sessions on one day.
pub fn focus_minutes_on(conn: &Connection, date: NaiveDate) -> Result<i64> {
    conn.query_row(
        "SELECT COALESCE(SUM((ended_at - started_at) / 60), 0)
         FROM entries
         WHERE kind = 'focus' AND outcome = 'completed' AND occurred_on = ?1",
        params![date.to_string()],
        |r| r.get(0),
    )
}

// ------------------------------------------------------------ weather --

pub fn latest_weather_json(conn: &Connection) -> Result<Option<(i64, String)>> {
    conn.query_row(
        "SELECT fetched_at, payload_json FROM weather_cache ORDER BY fetched_at DESC LIMIT 1",
        [],
        |r| Ok((r.get(0)?, r.get(1)?)),
    )
    .optional()
}

pub fn insert_weather_json(conn: &Connection, fetched_at: i64, payload_json: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO weather_cache (fetched_at, payload_json) VALUES (?1, ?2)",
        params![fetched_at, payload_json],
    )?;
    Ok(())
}

// ------------------------------------------------------------ ledger agg --

pub fn total_points(conn: &Connection) -> Result<i64> {
    conn.query_row("SELECT COALESCE(SUM(points), 0) FROM ethos_ledger", [], |r| r.get(0))
}

/// The date of the very first ledger entry — the true start of the Path,
/// rather than an arbitrary lookback window.
pub fn earliest_ledger_date(conn: &Connection) -> Result<Option<NaiveDate>> {
    let date_str: Option<String> = conn.query_row(
        "SELECT date(MIN(created_at), 'unixepoch') FROM ethos_ledger",
        [],
        |r| r.get(0),
    )?;
    Ok(date_str.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()))
}

pub fn points_by_day(conn: &Connection, since: NaiveDate) -> Result<BTreeMap<NaiveDate, i64>> {
    let mut stmt = conn.prepare(
        "SELECT date(created_at, 'unixepoch') AS d, SUM(points)
         FROM ethos_ledger WHERE created_at >= ?1 GROUP BY d",
    )?;
    let since_ts = since
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        .timestamp();
    let rows = stmt.query_map(params![since_ts], |r| {
        let date_str: String = r.get(0)?;
        let points: i64 = r.get(1)?;
        Ok((date_str, points))
    })?;
    let mut map = BTreeMap::new();
    for row in rows {
        let (date_str, points) = row?;
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            map.insert(date, points);
        }
    }
    Ok(map)
}

/// Focus minutes per pillar per day, since `since` (inclusive).
pub fn focus_minutes_by_pillar_day(
    conn: &Connection,
    since: NaiveDate,
) -> Result<Vec<(NaiveDate, String, i64)>> {
    let mut stmt = conn.prepare(
        "SELECT e.occurred_on, a.pillar_id,
                SUM(CASE WHEN e.outcome = 'completed' THEN (e.ended_at - e.started_at) / 60 ELSE 0 END)
         FROM entries e
         JOIN actions a ON a.id = e.action_id
         WHERE e.kind = 'focus' AND e.occurred_on >= ?1 AND e.ended_at IS NOT NULL
         GROUP BY e.occurred_on, a.pillar_id",
    )?;
    let rows = stmt.query_map(params![since.to_string()], |r| {
        let date_str: String = r.get(0)?;
        let pillar_id: String = r.get(1)?;
        let minutes: i64 = r.get(2)?;
        Ok((date_str, pillar_id, minutes))
    })?;
    let mut out = Vec::new();
    for row in rows {
        let (date_str, pillar_id, minutes) = row?;
        if let Ok(date) = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
            out.push((date, pillar_id, minutes));
        }
    }
    Ok(out)
}

pub fn week_start(date: NaiveDate) -> NaiveDate {
    let weekday = date.weekday().num_days_from_monday();
    date - Duration::days(weekday as i64)
}
