mod commands;
mod db;
mod domain;

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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
