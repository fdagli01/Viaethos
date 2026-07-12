use std::collections::BTreeMap;

use chrono::{Datelike, Duration, Local, NaiveDate};
use rusqlite::{params, Connection, OptionalExtension, Result};
use uuid::Uuid;

use crate::domain::models::{Action, ActionKind, Entry, Pillar, Schedule};

pub fn today() -> NaiveDate {
    Local::now().date_naive()
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
                started_at, ended_at, outcome, reflection, created_at
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
        created_at: r.get(10)?,
    })
}

pub fn get_entry(conn: &Connection, entry_id: &str) -> Result<Entry> {
    conn.query_row(
        "SELECT id, action_id, kind, occurred_on, intention, planned_minutes,
                started_at, ended_at, outcome, reflection, created_at
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
             started_at, ended_at, outcome, reflection, created_at)
         VALUES (?1, ?2, 'tick', ?3, NULL, NULL, ?4, ?4, 'completed', NULL, ?4)",
        params![id, action_id, today().to_string(), now],
    )?;
    Ok(id)
}

pub fn start_focus(
    conn: &Connection,
    action_id: &str,
    intention: Option<&str>,
    planned_minutes: i64,
) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let now = now_ts();
    conn.execute(
        "INSERT INTO entries
            (id, action_id, kind, occurred_on, intention, planned_minutes,
             started_at, ended_at, outcome, reflection, created_at)
         VALUES (?1, ?2, 'focus', ?3, ?4, ?5, ?6, NULL, NULL, NULL, ?6)",
        params![id, action_id, today().to_string(), intention, planned_minutes, now],
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
    action_id: &str,
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

// ------------------------------------------------------------ ledger agg --

pub fn total_points(conn: &Connection) -> Result<i64> {
    conn.query_row("SELECT COALESCE(SUM(points), 0) FROM ethos_ledger", [], |r| r.get(0))
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
