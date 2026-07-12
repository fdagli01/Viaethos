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
    let today = repo::today(conn);
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
    let today = repo::today(&conn);
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
    lesson_id: Option<String>,
) -> Result<TodayView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::start_focus(
        &conn,
        &action_id,
        intention.as_deref(),
        planned_minutes,
        lesson_id.as_deref(),
    )
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

        let today = repo::today(&conn);
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

        if let Some(lesson_id) = &entry.lesson_id {
            repo::complete_lesson(&conn, lesson_id).map_err(|e| e.to_string())?;
        }
    }

    let view = build_today_view(&conn)?;
    let _ = app.emit("entry-logged", &entry.action_id);
    Ok(view)
}

// ---------------------------------------------------------------- Weather --

const WEATHER_TTL_SECONDS: i64 = 20 * 60;
const DEFAULT_LAT: f64 = 41.0082;
const DEFAULT_LON: f64 = 28.9784;

fn weather_location(conn: &rusqlite::Connection) -> (f64, f64) {
    let lat = repo::get_setting(conn, "weather_lat")
        .ok()
        .flatten()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(DEFAULT_LAT);
    let lon = repo::get_setting(conn, "weather_lon")
        .ok()
        .flatten()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(DEFAULT_LON);
    (lat, lon)
}

/// Real weather from Open-Meteo, cached in `weather_cache`. Falls back to the
/// last known snapshot (marked `stale`) when offline; returns `None` only if
/// no snapshot has ever been fetched. Never holds the DB lock across the
/// network await — the mutex guard is dropped before any `.await`.
#[tauri::command]
pub async fn get_weather(
    db: State<'_, DbState>,
) -> Result<Option<crate::weather::WeatherSnapshot>, String> {
    let (lat, lon, cached) = {
        let conn = db.0.lock().map_err(|e| e.to_string())?;
        let (lat, lon) = weather_location(&conn);
        let cached = repo::latest_weather_json(&conn).map_err(|e| e.to_string())?;
        (lat, lon, cached)
    };

    let now = repo::now_ts();
    if let Some((fetched_at, payload)) = &cached {
        if now - fetched_at < WEATHER_TTL_SECONDS {
            let snapshot: crate::weather::WeatherSnapshot =
                serde_json::from_str(payload).map_err(|e| e.to_string())?;
            return Ok(Some(snapshot));
        }
    }

    match crate::weather::fetch(lat, lon).await {
        Ok(snapshot) => {
            let payload = serde_json::to_string(&snapshot).map_err(|e| e.to_string())?;
            let conn = db.0.lock().map_err(|e| e.to_string())?;
            repo::insert_weather_json(&conn, snapshot.fetched_at, &payload)
                .map_err(|e| e.to_string())?;
            Ok(Some(snapshot))
        }
        Err(_) => match cached {
            Some((_, payload)) => {
                let mut snapshot: crate::weather::WeatherSnapshot =
                    serde_json::from_str(&payload).map_err(|e| e.to_string())?;
                snapshot.stale = true;
                Ok(Some(snapshot))
            }
            None => Ok(None),
        },
    }
}

// ------------------------------------------------------------------ Sofra --

const DEFAULT_KCAL_BUDGET: f64 = 2000.0;

#[derive(Serialize)]
pub struct MealsView {
    pub meals: Vec<crate::domain::models::Meal>,
    pub kcal_total: f64,
    pub kcal_budget: f64,
}

fn build_meals_view(conn: &rusqlite::Connection) -> Result<MealsView, String> {
    let meals = repo::list_meals_on(conn, repo::today(conn)).map_err(|e| e.to_string())?;
    let kcal_total = repo::kcal_today(conn).map_err(|e| e.to_string())?;
    let kcal_budget = repo::get_setting(conn, "calorie_budget")
        .map_err(|e| e.to_string())?
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(DEFAULT_KCAL_BUDGET);
    Ok(MealsView {
        meals,
        kcal_total,
        kcal_budget,
    })
}

#[tauri::command]
pub fn search_food_items(
    db: State<DbState>,
    query: String,
) -> Result<Vec<crate::domain::models::FoodItem>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::search_food_items(&conn, &query).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn add_food_item(
    db: State<DbState>,
    name: String,
    kcal_per_100g: f64,
    protein_per_100g: f64,
    carb_per_100g: f64,
    fat_per_100g: f64,
) -> Result<crate::domain::models::FoodItem, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let id = repo::add_food_item(
        &conn,
        &name,
        kcal_per_100g,
        protein_per_100g,
        carb_per_100g,
        fat_per_100g,
    )
    .map_err(|e| e.to_string())?;
    Ok(crate::domain::models::FoodItem {
        id,
        name,
        kcal_per_100g,
        protein_per_100g,
        carb_per_100g,
        fat_per_100g,
        user_defined: true,
    })
}

#[tauri::command]
pub fn get_meals_today(db: State<DbState>) -> Result<MealsView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    build_meals_view(&conn)
}

#[tauri::command]
pub fn add_meal(
    app: tauri::AppHandle,
    db: State<DbState>,
    time_slot: String,
    name: String,
    kcal: f64,
    protein_g: f64,
    carb_g: f64,
    fat_g: f64,
    note: Option<String>,
) -> Result<MealsView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let meal_id = repo::add_meal(
        &conn,
        &time_slot,
        &name,
        kcal,
        protein_g,
        carb_g,
        fat_g,
        note.as_deref(),
    )
    .map_err(|e| e.to_string())?;

    let pillars = repo::list_pillars(&conn).map_err(|e| e.to_string())?;
    if let Some(body) = pillars.iter().find(|p| p.name == "Body") {
        repo::insert_ledger(&conn, &body.id, None, None, ethos::meal_points(), "meal_logged")
            .map_err(|e| e.to_string())?;
    }

    let view = build_meals_view(&conn)?;
    let _ = app.emit("entry-logged", &meal_id);
    Ok(view)
}

#[tauri::command]
pub fn delete_meal(db: State<DbState>, meal_id: String) -> Result<MealsView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::delete_meal(&conn, &meal_id).map_err(|e| e.to_string())?;
    build_meals_view(&conn)
}

// ----------------------------------------------------------------- Manage --

#[tauri::command]
pub fn update_pillar(
    db: State<DbState>,
    pillar_id: String,
    name: String,
    color_token: String,
) -> Result<Vec<crate::domain::models::Pillar>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::update_pillar(&conn, &pillar_id, &name, &color_token).map_err(|e| e.to_string())?;
    repo::list_pillars(&conn).map_err(|e| e.to_string())
}

#[derive(Serialize)]
pub struct ActionAdminView {
    pub id: String,
    pub pillar_id: String,
    pub name: String,
    pub kind: String,
    pub default_minutes: Option<i64>,
    pub schedule_type: String,
    pub times_per_week: Option<u8>,
    pub target_per_day: i64,
    pub archived: bool,
}

fn schedule_to_admin(schedule: &crate::domain::models::Schedule) -> (String, Option<u8>) {
    use crate::domain::models::Schedule;
    match schedule {
        Schedule::Daily => ("daily".to_string(), None),
        Schedule::Weekdays => ("weekdays".to_string(), None),
        Schedule::Days { .. } => ("daily".to_string(), None), // not editable in Manage v1
        Schedule::TimesPerWeek { n } => ("times_per_week".to_string(), Some(*n)),
    }
}

fn schedule_from_admin(schedule_type: &str, times_per_week: Option<u8>) -> crate::domain::models::Schedule {
    use crate::domain::models::Schedule;
    match schedule_type {
        "weekdays" => Schedule::Weekdays,
        "times_per_week" => Schedule::TimesPerWeek {
            n: times_per_week.unwrap_or(3),
        },
        _ => Schedule::Daily,
    }
}

fn build_action_admin_views(conn: &rusqlite::Connection) -> Result<Vec<ActionAdminView>, String> {
    let actions = repo::list_all_actions(conn).map_err(|e| e.to_string())?;
    Ok(actions
        .into_iter()
        .map(|(a, archived)| {
            let (schedule_type, times_per_week) = schedule_to_admin(&a.schedule);
            ActionAdminView {
                id: a.id,
                pillar_id: a.pillar_id,
                name: a.name,
                kind: a.kind.as_str().to_string(),
                default_minutes: a.default_minutes,
                schedule_type,
                times_per_week,
                target_per_day: a.target_per_day,
                archived,
            }
        })
        .collect())
}

#[tauri::command]
pub fn get_manage_actions(db: State<DbState>) -> Result<Vec<ActionAdminView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    build_action_admin_views(&conn)
}

#[tauri::command]
pub fn add_action(
    db: State<DbState>,
    pillar_id: String,
    name: String,
    kind: String,
    default_minutes: Option<i64>,
    schedule_type: String,
    times_per_week: Option<u8>,
    target_per_day: i64,
) -> Result<Vec<ActionAdminView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let schedule = schedule_from_admin(&schedule_type, times_per_week);
    repo::add_action(
        &conn,
        &pillar_id,
        &name,
        crate::domain::models::ActionKind::from_str(&kind),
        default_minutes,
        &schedule,
        target_per_day,
    )
    .map_err(|e| e.to_string())?;
    build_action_admin_views(&conn)
}

#[tauri::command]
pub fn update_action(
    db: State<DbState>,
    action_id: String,
    name: String,
    default_minutes: Option<i64>,
    schedule_type: String,
    times_per_week: Option<u8>,
    target_per_day: i64,
) -> Result<Vec<ActionAdminView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let schedule = schedule_from_admin(&schedule_type, times_per_week);
    repo::update_action(&conn, &action_id, &name, default_minutes, &schedule, target_per_day)
        .map_err(|e| e.to_string())?;
    build_action_admin_views(&conn)
}

#[tauri::command]
pub fn set_action_archived(
    db: State<DbState>,
    action_id: String,
    archived: bool,
) -> Result<Vec<ActionAdminView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::set_action_archived(&conn, &action_id, archived).map_err(|e| e.to_string())?;
    build_action_admin_views(&conn)
}

// ---------------------------------------------------------------- Settings --

#[derive(Serialize)]
pub struct SettingsView {
    pub day_boundary_hour: i64,
    pub calorie_budget: f64,
    pub weather_lat: f64,
    pub weather_lon: f64,
}

fn build_settings_view(conn: &rusqlite::Connection) -> Result<SettingsView, String> {
    let day_boundary_hour = repo::get_setting(conn, "day_boundary_hour")
        .map_err(|e| e.to_string())?
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(0);
    let calorie_budget = repo::get_setting(conn, "calorie_budget")
        .map_err(|e| e.to_string())?
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(DEFAULT_KCAL_BUDGET);
    let (weather_lat, weather_lon) = weather_location(conn);
    Ok(SettingsView {
        day_boundary_hour,
        calorie_budget,
        weather_lat,
        weather_lon,
    })
}

#[tauri::command]
pub fn get_settings(db: State<DbState>) -> Result<SettingsView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    build_settings_view(&conn)
}

#[tauri::command]
pub fn update_settings(
    db: State<DbState>,
    day_boundary_hour: i64,
    calorie_budget: f64,
    weather_lat: f64,
    weather_lon: f64,
) -> Result<SettingsView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::set_setting(&conn, "day_boundary_hour", &day_boundary_hour.to_string())
        .map_err(|e| e.to_string())?;
    repo::set_setting(&conn, "calorie_budget", &calorie_budget.to_string())
        .map_err(|e| e.to_string())?;
    repo::set_setting(&conn, "weather_lat", &weather_lat.to_string()).map_err(|e| e.to_string())?;
    repo::set_setting(&conn, "weather_lon", &weather_lon.to_string()).map_err(|e| e.to_string())?;
    build_settings_view(&conn)
}

// -------------------------------------------------------------- Memento Mori --

/// Not user-configurable in v0.6 — a single fixed horizon keeps the grid a
/// stable, honest reference rather than a number to negotiate with.
const LIFE_EXPECTANCY_YEARS: i64 = 80;

#[derive(Serialize)]
pub struct MementoMoriView {
    pub birth_date: Option<String>,
    pub weeks_lived: Option<i64>,
    pub weeks_total: i64,
}

fn build_memento_mori_view(conn: &rusqlite::Connection) -> Result<MementoMoriView, String> {
    let birth_date = repo::get_setting(conn, "birth_date").map_err(|e| e.to_string())?;
    let weeks_total = LIFE_EXPECTANCY_YEARS * 52;
    let weeks_lived = birth_date
        .as_deref()
        .and_then(|s| chrono::NaiveDate::parse_from_str(s, "%Y-%m-%d").ok())
        .map(|birth| (repo::today(conn) - birth).num_days() / 7);
    Ok(MementoMoriView {
        birth_date,
        weeks_lived,
        weeks_total,
    })
}

#[tauri::command]
pub fn get_memento_mori(db: State<DbState>) -> Result<MementoMoriView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    build_memento_mori_view(&conn)
}

#[tauri::command]
pub fn set_birth_date(db: State<DbState>, birth_date: String) -> Result<MementoMoriView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::set_setting(&conn, "birth_date", &birth_date).map_err(|e| e.to_string())?;
    build_memento_mori_view(&conn)
}

// ------------------------------------------------------------------- Uyku --

#[tauri::command]
pub fn get_last_sleep(
    db: State<DbState>,
) -> Result<Option<crate::domain::models::SleepLog>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::get_sleep_on(&conn, repo::today(&conn)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn log_sleep(
    app: tauri::AppHandle,
    db: State<DbState>,
    date: String,
    bed_at: String,
    woke_at: String,
    quality: i64,
) -> Result<crate::domain::models::SleepLog, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let date_parsed = chrono::NaiveDate::parse_from_str(&date, "%Y-%m-%d")
        .map_err(|e| e.to_string())?;
    let already_logged = repo::get_sleep_on(&conn, date_parsed)
        .map_err(|e| e.to_string())?
        .is_some();

    repo::log_sleep(&conn, &date, &bed_at, &woke_at, quality).map_err(|e| e.to_string())?;

    if !already_logged {
        let pillars = repo::list_pillars(&conn).map_err(|e| e.to_string())?;
        if let Some(life) = pillars.iter().find(|p| p.name == "Life") {
            repo::insert_ledger(&conn, &life.id, None, None, ethos::sleep_log_points(), "sleep_logged")
                .map_err(|e| e.to_string())?;
        }
    }

    let log = repo::get_sleep_on(&conn, date_parsed)
        .map_err(|e| e.to_string())?
        .ok_or("sleep log not found after insert")?;
    let _ = app.emit("entry-logged", &log.id);
    Ok(log)
}

// --------------------------------------------------------------- Müfredat --

#[derive(Serialize)]
pub struct CourseView {
    pub id: String,
    pub name: String,
    pub pillar_id: String,
    pub action_id: String,
    pub color_token: String,
    pub target_hours_week: f64,
    pub done_lessons: i64,
    pub total_lessons: i64,
    pub due_lessons: Vec<crate::domain::models::Lesson>,
}

fn build_course_views(conn: &rusqlite::Connection) -> Result<Vec<CourseView>, String> {
    let courses = repo::list_courses(conn).map_err(|e| e.to_string())?;
    courses
        .into_iter()
        .map(|c| {
            let (done, total) = repo::course_progress(conn, &c.id).map_err(|e| e.to_string())?;
            let due = repo::lessons_due(conn, &c.id).map_err(|e| e.to_string())?;
            Ok(CourseView {
                id: c.id,
                name: c.name,
                pillar_id: c.pillar_id,
                action_id: c.action_id,
                color_token: c.color_token,
                target_hours_week: c.target_hours_week,
                done_lessons: done,
                total_lessons: total,
                due_lessons: due,
            })
        })
        .collect()
}

#[tauri::command]
pub fn get_curriculum(db: State<DbState>) -> Result<Vec<CourseView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    build_course_views(&conn)
}

#[tauri::command]
pub fn add_course(
    db: State<DbState>,
    name: String,
    pillar_id: String,
    target_hours_week: f64,
) -> Result<Vec<CourseView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::add_course(&conn, &name, &pillar_id, target_hours_week).map_err(|e| e.to_string())?;
    build_course_views(&conn)
}

#[tauri::command]
pub fn add_lesson(
    db: State<DbState>,
    course_id: String,
    title: String,
    planned_on: String,
) -> Result<Vec<CourseView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::add_lesson(&conn, &course_id, &title, &planned_on).map_err(|e| e.to_string())?;
    build_course_views(&conn)
}

/// Standalone completion (not via a Focus Session) — still earns a small
/// flat bonus so marking a lesson done isn't free, and still schedules the
/// next spaced-repetition review exactly like the Focus-Session path.
#[tauri::command]
pub fn complete_lesson(
    app: tauri::AppHandle,
    db: State<DbState>,
    lesson_id: String,
) -> Result<Vec<CourseView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let lesson = repo::get_lesson(&conn, &lesson_id).map_err(|e| e.to_string())?;
    let course = repo::get_course(&conn, &lesson.course_id).map_err(|e| e.to_string())?;
    repo::complete_lesson(&conn, &lesson_id).map_err(|e| e.to_string())?;
    repo::insert_ledger(
        &conn,
        &course.pillar_id,
        None,
        None,
        ethos::lesson_points(),
        "lesson_complete",
    )
    .map_err(|e| e.to_string())?;
    let views = build_course_views(&conn)?;
    let _ = app.emit("entry-logged", &lesson_id);
    Ok(views)
}

#[tauri::command]
pub fn skip_lesson(db: State<DbState>, lesson_id: String) -> Result<Vec<CourseView>, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    repo::skip_lesson(&conn, &lesson_id).map_err(|e| e.to_string())?;
    build_course_views(&conn)
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
pub struct SleepDay {
    pub date: String,
    pub hours: f64,
    pub quality_1_5: Option<i64>,
}

#[derive(Serialize)]
pub struct LedgerStats {
    pub total_points: i64,
    pub points_trend: Vec<DayPoint>,
    pub focus_by_pillar_day: Vec<PillarFocusDay>,
    pub streaks: Vec<StreakRow>,
    pub sleep_by_day: Vec<SleepDay>,
}

#[tauri::command]
pub fn get_ledger_stats(db: State<DbState>) -> Result<LedgerStats, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let today = repo::today(&conn);

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

    let sleep_since = today - Duration::days(6);
    let sleep_logs = repo::sleep_by_day(&conn, sleep_since).map_err(|e| e.to_string())?;
    let mut sleep_by_day = Vec::new();
    let mut d = sleep_since;
    while d <= today {
        let log = sleep_logs.iter().find(|s| s.date == d.to_string());
        sleep_by_day.push(SleepDay {
            date: d.to_string(),
            hours: log.map(|s| s.hours()).unwrap_or(0.0),
            quality_1_5: log.map(|s| s.quality_1_5),
        });
        d += Duration::days(1);
    }

    Ok(LedgerStats {
        total_points,
        points_trend,
        focus_by_pillar_day,
        streaks,
        sleep_by_day,
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
pub struct CourseBand {
    pub name: String,
    pub color_token: String,
    pub ratio: f64, // done / total, 0 when no lessons yet
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
    pub kcal_today: f64,
    pub kcal_budget: f64,
    pub course_bands: Vec<CourseBand>,
    pub sleep_hours: Option<f64>,
    pub sleep_quality: Option<i64>,
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
    let today = repo::today(&conn);

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

    let kcal_today = repo::kcal_today(&conn).map_err(|e| e.to_string())?;
    let kcal_budget = repo::get_setting(&conn, "calorie_budget")
        .map_err(|e| e.to_string())?
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(DEFAULT_KCAL_BUDGET);

    let courses = repo::list_courses(&conn).map_err(|e| e.to_string())?;
    let course_bands: Vec<CourseBand> = courses
        .iter()
        .take(4)
        .map(|c| {
            let (done, total) = repo::course_progress(&conn, &c.id).unwrap_or((0, 0));
            let ratio = if total > 0 { done as f64 / total as f64 } else { 0.0 };
            CourseBand {
                name: c.name.clone(),
                color_token: c.color_token.clone(),
                ratio,
            }
        })
        .collect();

    let last_sleep = repo::get_sleep_on(&conn, today).map_err(|e| e.to_string())?;
    let (sleep_hours, sleep_quality) = match &last_sleep {
        Some(s) => (Some(s.hours()), Some(s.quality_1_5)),
        None => (None, None),
    };

    Ok(QuietModeView {
        honored_today,
        due_today,
        focus_minutes_today,
        total_points: today_view.total_points,
        best_streak,
        inner_weather,
        path,
        milestones,
        kcal_today,
        kcal_budget,
        course_bands,
        sleep_hours,
        sleep_quality,
    })
}

#[tauri::command]
pub fn set_inner_weather(db: State<DbState>, weather: String) -> Result<(), String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let today = repo::today(&conn);
    repo::set_setting(&conn, &inner_weather_key(today), &weather).map_err(|e| e.to_string())
}

// ------------------------------------------------------------------- Path --

const STREAK_MILESTONE_THRESHOLDS: [i64; 3] = [7, 30, 100];
const POINTS_MILESTONE_STEP: i64 = 1000;

#[derive(Serialize)]
pub struct PathMilestone {
    pub date: String,
    pub label: String,
    pub kind: String, // "streak" | "points"
}

#[derive(Serialize)]
pub struct PathHistoryView {
    pub days: Vec<PathDay>,
    pub milestones: Vec<PathMilestone>,
}

fn points_milestone_dates(
    daily_points: &std::collections::BTreeMap<chrono::NaiveDate, i64>,
    step: i64,
) -> Vec<(chrono::NaiveDate, i64)> {
    let mut hits = Vec::new();
    let mut cumulative = 0i64;
    let mut next = step;
    for (date, points) in daily_points {
        cumulative += points;
        while cumulative >= next {
            hits.push((*date, next));
            next += step;
        }
    }
    hits
}

/// The full, historical Path: every day back to `days` ago (or the true
/// start of the ledger, whichever is later) as a pillar-mix stone, plus
/// milestone markers — the first time any action's streak crossed 7/30/100
/// days, and every 1,000-point cumulative crossing. Each milestone is a
/// one-time life event: once recorded it's never re-fired, even if a streak
/// later breaks and rebuilds.
#[tauri::command]
pub fn get_path_history(db: State<DbState>, days: i64) -> Result<PathHistoryView, String> {
    let conn = db.0.lock().map_err(|e| e.to_string())?;
    let today = repo::today(&conn);
    let since = today - Duration::days((days - 1).max(0));

    let pillars = repo::list_pillars(&conn).map_err(|e| e.to_string())?;
    let mix_by_day = repo::pillar_mix_by_day(&conn, since).map_err(|e| e.to_string())?;
    let mut path_days = Vec::new();
    let mut d = since;
    while d <= today {
        let pillar_ids = mix_by_day.get(&d).cloned().unwrap_or_default();
        let pillar_colors = pillars
            .iter()
            .filter(|p| pillar_ids.contains(&p.id))
            .map(|p| p.color_token.clone())
            .collect();
        path_days.push(PathDay {
            date: d.to_string(),
            pillar_colors,
        });
        d += Duration::days(1);
    }

    let mut milestones = Vec::new();

    let actions = repo::list_actions(&conn).map_err(|e| e.to_string())?;
    let long_history_since = today - Duration::days(3650);
    for action in &actions {
        let counts = repo::completed_counts_since(&conn, &action.id, long_history_since)
            .map_err(|e| e.to_string())?;
        let hits = streak::streak_milestone_dates(
            &action.schedule,
            action.target_per_day,
            &counts,
            &STREAK_MILESTONE_THRESHOLDS,
        );
        for (date, n) in hits {
            if date >= since && date <= today {
                milestones.push(PathMilestone {
                    date: date.to_string(),
                    label: format!("{} · {n}-day streak", action.name),
                    kind: "streak".to_string(),
                });
            }
        }
    }

    if let Some(earliest) = repo::earliest_ledger_date(&conn).map_err(|e| e.to_string())? {
        let daily_points = repo::points_by_day(&conn, earliest).map_err(|e| e.to_string())?;
        for (date, n) in points_milestone_dates(&daily_points, POINTS_MILESTONE_STEP) {
            if date >= since && date <= today {
                milestones.push(PathMilestone {
                    date: date.to_string(),
                    label: format!("{n} Ethos Points"),
                    kind: "points".to_string(),
                });
            }
        }
    }

    milestones.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(PathHistoryView {
        days: path_days,
        milestones,
    })
}
