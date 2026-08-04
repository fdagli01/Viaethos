mod ai;
mod commands;
mod db;
mod domain;
mod weather;

use std::sync::Mutex;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }

            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("resolve app data dir");
            let conn = db::init(&app_data_dir);
            app.manage(db::DbState(Mutex::new(conn)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_today,
            commands::complete_tick,
            commands::start_focus,
            commands::end_focus,
            commands::get_ledger_stats,
            commands::get_quiet_mode,
            commands::set_inner_weather,
            commands::get_tasks,
            commands::add_task,
            commands::complete_task,
            commands::delete_task,
            commands::get_weather,
            commands::search_food_items,
            commands::add_food_item,
            commands::get_meals_today,
            commands::add_meal,
            commands::delete_meal,
            commands::get_curriculum,
            commands::add_course,
            commands::add_lesson,
            commands::complete_lesson,
            commands::skip_lesson,
            commands::get_last_sleep,
            commands::log_sleep,
            commands::get_memento_mori,
            commands::set_birth_date,
            commands::get_settings,
            commands::update_settings,
            commands::update_pillar,
            commands::get_manage_actions,
            commands::add_action,
            commands::update_action,
            commands::set_action_archived,
            commands::get_path_history,
            commands::get_course_detail,
            commands::reorder_lessons,
            commands::get_schedule,
            commands::add_schedule_block,
            commands::update_schedule_block,
            commands::delete_schedule_block,
            commands::suggest_schedule,
            commands::set_ai_api_key,
            commands::get_meal_presets,
            commands::reset_all_data,
            commands::generate_synthetic_data,
            commands::clear_synthetic_data,
            commands::has_synthetic_data,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
