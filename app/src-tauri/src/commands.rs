use chrono::Duration;
use serde::Serialize;
use tauri::{Emitter, State};

use crate::db::{repo, DbState};
use crate::domain::{ethos, schedule, streak};

#[derive(Serialize)]
pub struct ActiveSessionView {
    pub entry_id: String,
    pub intention: Option<String>,
    pub started_at: i64,
    pub planned_minutes: i64,
}

#[derive(Serialize)]
pub struct ActionView {
    pub id: String,
    pub pillar_id: String,
    pub name: String,
    pub kind: String,
    pub default_minutes: Option<i64>,
    pub target_per_day: i64,
    pub today_count: i64,
    pub streak: i64,
    pub best_streak: i64,
    pub due_today: bool,
    pub active_session: Option<ActiveSessionView>,
}

#[derive(Serialize)]
pub struct PillarView {
    pub id: String,
    pub name: String,
    pub color_token: String,
    pub actions: Vec<ActionView>,
}

#[derive(Serialize)]
pub struct TodayView {
    pub pillars: Vec<PillarView>,
    pub total_points: i64,
}

fn build_today_view(conn: &rusqlite::Connection) -> Result<TodayView, String> {
    let pillars = repo::list_pillars(conn).map_err(|e| e.to_string())?;
    let actions = repo::list_actions(conn).map_err(|e| e.to_string())?;
    let today = repo::today();
    let history_since = today - Duration::days(400);

    let mut pillar_views: Vec<PillarView> = pillars
        .into_iter()
        .map(|p| PillarView {
            id: p.id,
            name: p.name,
            color_token: p.color_token,
            actions: Vec::new(),
        })
        .collect();

    for action in actions {
        let counts = repo::completed_counts_since(conn, &action.id, history_since)
            .map_err(|e| e.to_string())?;
        let today_count = counts.get(&today).copied().unwrap_or(0);
        let current = streak::current_streak(&action.schedule, action.target_per_day, &counts, today);
        let best = streak::best_streak(&action.schedule, action.target_per_day, &counts);

        let week_start = repo::week_start(today);
        let entries_this_week: i64 = counts
            .iter()
            .filter(|(d, _)| **d >= week_start)
            .map(|(_, c)| c)
            .sum();
        let due_today = schedule::is_due_on(&action.schedule, today, entries_this_week as u32);

        let active_session = if action.kind.as_str() == "focus" {
            repo::active_focus_entry(conn, &action.id)
                .map_err(|e| e.to_string())?
                .map(|e| ActiveSessionView {
                    entry_id: e.id,
                    intention: e.intention,
                    started_at: e.started_at.unwrap_or(0),
                    planned_minutes: e.planned_minutes.unwrap_or(action.default_minutes.unwrap_or(25)),
                })
        } else {
            None
        };

        let view = ActionView {
            id: action.id,
            pillar_id: action.pillar_id.clone(),
            name: action.name,
            kind: action.kind.as_str().to_string(),
            default_minutes: action.default_minutes,
            target_per_day: action.target_per_day,
            today_count,
            streak: current,
            best_streak: best,
            due_today,
            active_session,
        };

        if let Some(pv) = pillar_views.iter_mut().find(|p| p.id == view.pillar_id) {
            pv.actions.push(view);
        }
    }

    let total_points = repo::total_points(conn).map_err(|e| e.to_string())?;

    Ok(TodayView {
        pillars: pillar_views,
        total_points,
    })
}

#[tauri::command]
pub fn get_today(db: State<DbState>) -> Result<TodayView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    build_today_view(&conn)
}

#[tauri::command]
pub fn complete_tick(app: tauri::AppHandle, db: State<DbState>, action_id: String) -> Result<TodayView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let action = repo::get_action(&conn, &action_id).map_err(|e| e.to_string())?;
    let today = repo::today();
    let history_since = today - Duration::days(400);
    let counts_before =
        repo::completed_counts_since(&conn, &action_id, history_since).map_err(|e| e.to_string())?;
    let streak_before =
        streak::current_streak(&action.schedule, action.target_per_day, &counts_before, today);

    let entry_id = repo::insert_tick(&conn, &action_id).map_err(|e| e.to_string())?;
    let points = ethos::tick_points(streak_before);
    repo::insert_ledger(
        &conn,
        &action.pillar_id,
        Some(&action_id),
        Some(&entry_id),
        points,
        "tick_complete",
    )
    .map_err(|e| e.to_string())?;

    let view = build_today_view(&conn)?;
    let _ = app.emit("entry-logged", &action_id);
    Ok(view)
}

#[tauri::command]
pub fn start_focus(
    db: State<DbState>,
    action_id: String,
    intention: Option<String>,
    planned_minutes: i64,
) -> Result<TodayView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::start_focus(&conn, &action_id, intention.as_deref(), planned_minutes)
        .map_err(|e| e.to_string())?;
    build_today_view(&conn)
}

#[tauri::command]
pub fn end_focus(
    app: tauri::AppHandle,
    db: State<DbState>,
    entry_id: String,
    outcome: String,
    reflection: Option<String>,
) -> Result<TodayView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let entry = repo::end_focus(&conn, &entry_id, &outcome, reflection.as_deref())
        .map_err(|e| e.to_string())?;
    let action = repo::get_action(&conn, &entry.action_id).map_err(|e| e.to_string())?;

    if outcome == "completed" {
        let started = entry.started_at.unwrap_or(0);
        let ended = entry.ended_at.unwrap_or(started);
        let minutes = ((ended - started) / 60).max(0);

        let today = repo::today();
        let history_since = today - Duration::days(400);
        let counts = repo::completed_counts_since(&conn, &action.id, history_since)
            .map_err(|e| e.to_string())?;
        let streak_before =
            streak::current_streak(&action.schedule, action.target_per_day, &counts, today);
        let sealed = entry.intention.is_some() && entry.reflection.is_some();
        let points = ethos::focus_points(minutes, streak_before, sealed);

        repo::insert_ledger(
            &conn,
            &action.pillar_id,
            Some(&action.id),
            Some(&entry_id),
            points,
            "focus_complete",
        )
        .map_err(|e| e.to_string())?;
    }

    let view = build_today_view(&conn)?;
    let _ = app.emit("entry-logged", &entry.action_id);
    Ok(view)
}

// ------------------------------------------------------------------ Tasks --

#[tauri::command]
pub fn get_tasks(db: State<DbState>) -> Result<Vec<crate::domain::models::Task>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::list_open_tasks(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_task(
    db: State<DbState>,
    title: String,
    pillar_id: Option<String>,
    due_on: Option<String>,
) -> Result<Vec<crate::domain::models::Task>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::add_task(&conn, &title, pillar_id.as_deref(), due_on.as_deref())
        .map_err(|e| e.to_string())?;
    repo::list_open_tasks(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn complete_task(
    app: tauri::AppHandle,
    db: State<DbState>,
    task_id: String,
) -> Result<Vec<crate::domain::models::Task>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let task = repo::complete_task(&conn, &task_id).map_err(|e| e.to_string())?;
    if let Some(pillar_id) = &task.pillar_id {
        repo::insert_ledger(
            &conn,
            pillar_id,
            None,
            None,
            ethos::task_points(),
            "task_complete",
        )
        .map_err(|e| e.to_string())?;
    }
    let _ = app.emit("entry-logged", &task_id);
    repo::list_open_tasks(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_task(
    db: State<DbState>,
    task_id: String,
) -> Result<Vec<crate::domain::models::Task>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::delete_task(&conn, &task_id).map_err(|e| e.to_string())?;
    repo::list_open_tasks(&conn).map_err(|e| e.to_string())
}

// ------------------------------------------------------------- The Ledger --

#[derive(Serialize)]
pub struct DayPoint {
    pub date: String,
    pub points: i64,
}

#[derive(Serialize)]
pub struct PillarFocusDay {
    pub date: String,
    pub by_pillar: Vec<(String, i64)>, // (pillar_name, minutes)
}

#[derive(Serialize)]
pub struct StreakRow {
    pub action_name: String,
    pub pillar_name: String,
    pub color_token: String,
    pub kind: String,
    pub current: i64,
    pub best: i64,
}

#[derive(Serialize)]
pub struct LedgerStats {
    pub total_points: i64,
    pub points_trend: Vec<DayPoint>,
    pub focus_by_pillar_day: Vec<PillarFocusDay>,
    pub streaks: Vec<StreakRow>,
}

#[tauri::command]
pub fn get_ledger_stats(db: State<DbState>) -> Result<LedgerStats, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let today = repo::today();

    let trend_since = today - Duration::days(29);
    let points_map = repo::points_by_day(&conn, trend_since).map_err(|e| e.to_string())?;
    let mut points_trend = Vec::new();
    let mut d = trend_since;
    while d <= today {
        points_trend.push(DayPoint {
            date: d.to_string(),
            points: points_map.get(&d).copied().unwrap_or(0),
        });
        d += Duration::days(1);
    }

    let pillars = repo::list_pillars(&conn).map_err(|e| e.to_string())?;
    let focus_since = today - Duration::days(6);
    let raw_focus =
        repo::focus_minutes_by_pillar_day(&conn, focus_since).map_err(|e| e.to_string())?;
    let mut focus_by_pillar_day = Vec::new();
    let mut d = focus_since;
    while d <= today {
        let by_pillar = pillars
            .iter()
            .map(|p| {
                let minutes: i64 = raw_focus
                    .iter()
                    .filter(|(date, pid, _)| *date == d && pid == &p.id)
                    .map(|(_, _, m)| *m)
                    .sum();
                (p.name.clone(), minutes)
            })
            .collect();
        focus_by_pillar_day.push(PillarFocusDay {
            date: d.to_string(),
            by_pillar,
        });
        d += Duration::days(1);
    }

    let actions = repo::list_actions(&conn).map_err(|e| e.to_string())?;
    let history_since = today - Duration::days(400);
    let mut streaks = Vec::new();
    for action in &actions {
        let counts = repo::completed_counts_since(&conn, &action.id, history_since)
            .map_err(|e| e.to_string())?;
        let current =
            streak::current_streak(&action.schedule, action.target_per_day, &counts, today);
        let best = streak::best_streak(&action.schedule, action.target_per_day, &counts);
        if let Some(pillar) = pillars.iter().find(|p| p.id == action.pillar_id) {
            streaks.push(StreakRow {
                action_name: action.name.clone(),
                pillar_name: pillar.name.clone(),
                color_token: pillar.color_token.clone(),
                kind: action.kind.as_str().to_string(),
                current,
                best,
            });
        }
    }

    let total_points = repo::total_points(&conn).map_err(|e| e.to_string())?;

    Ok(LedgerStats {
        total_points,
        points_trend,
        focus_by_pillar_day,
        streaks,
    })
}

// -------------------------------------------------------------- Quiet Mode --

#[derive(Serialize)]
pub struct PathDay {
    pub date: String,
    pub pillar_colors: Vec<String>,
}

#[derive(Serialize)]
pub struct Milestone {
    pub title: String,
    pub color_token: String,
    pub overdue: bool,
}

#[derive(Serialize)]
pub struct QuietModeView {
    pub honored_today: i64,
    pub due_today: i64,
    pub focus_minutes_today: i64,
    pub total_points: i64,
    pub best_streak: i64,
    pub inner_weather: String,
    pub path: Vec<PathDay>,
    pub milestones: Vec<Milestone>,
}

/// Inner Weather is a manual, non-judgmental self-report — never inferred
/// from the ledger. See IDEAS.md's "yargisiz kayit" (judgment-free record)
/// principle: deriving mood from task completion would turn a rest day into
/// a verdict, which is exactly what this layer must not do.
fn inner_weather_key(date: chrono::NaiveDate) -> String {
    format!("inner_weather:{date}")
}

#[tauri::command]
pub fn get_quiet_mode(db: State<DbState>) -> Result<QuietModeView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let today_view = build_today_view(&conn)?;
    let today = repo::today();

    let due_today: i64 = today_view
        .pillars
        .iter()
        .flat_map(|p| &p.actions)
        .filter(|a| a.due_today)
        .count() as i64;
    let honored_today: i64 = today_view
        .pillars
        .iter()
        .flat_map(|p| &p.actions)
        .filter(|a| a.due_today && a.today_count >= a.target_per_day)
        .count() as i64;

    let focus_minutes_today = repo::focus_minutes_on(&conn, today).map_err(|e| e.to_string())?;

    let best_streak = today_view
        .pillars
        .iter()
        .flat_map(|p| &p.actions)
        .map(|a| a.best_streak)
        .max()
        .unwrap_or(0);

    let inner_weather = repo::get_setting(&conn, &inner_weather_key(today))
        .map_err(|e| e.to_string())?
        .unwrap_or_else(|| "clear".to_string());

    let pillars = repo::list_pillars(&conn).map_err(|e| e.to_string())?;
    let since = today - Duration::days(13);
    let mix_by_day = repo::pillar_mix_by_day(&conn, since).map_err(|e| e.to_string())?;
    let mut path = Vec::new();
    let mut d = since;
    while d <= today {
        let pillar_ids = mix_by_day.get(&d).cloned().unwrap_or_default();
        let pillar_colors = pillars
            .iter()
            .filter(|p| pillar_ids.contains(&p.id))
            .map(|p| p.color_token.clone())
            .collect();
        path.push(PathDay {
            date: d.to_string(),
            pillar_colors,
        });
        d += Duration::days(1);
    }

    let open_tasks = repo::list_open_tasks(&conn).map_err(|e| e.to_string())?;
    let today_str = today.to_string();
    let milestones: Vec<Milestone> = open_tasks
        .into_iter()
        .filter(|t| t.due_on.as_deref().map(|d| d <= today_str.as_str()).unwrap_or(true))
        .take(6)
        .map(|t| {
            let color_token = t
                .pillar_id
                .as_ref()
                .and_then(|pid| pillars.iter().find(|p| &p.id == pid))
                .map(|p| p.color_token.clone())
                .unwrap_or_else(|| "#8d88a0".to_string());
            let overdue = t.due_on.as_deref().map(|d| d < today_str.as_str()).unwrap_or(false);
            Milestone {
                title: t.title,
                color_token,
                overdue,
            }
        })
        .collect();

    Ok(QuietModeView {
        honored_today,
        due_today,
        focus_minutes_today,
        total_points: today_view.total_points,
        best_streak,
        inner_weather,
        path,
        milestones,
    })
}

#[tauri::command]
pub fn set_inner_weather(db: State<DbState>, weather: String) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let today = repo::today();
    repo::set_setting(&conn, &inner_weather_key(today), &weather).map_err(|e| e.to_string())
}
