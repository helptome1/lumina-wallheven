mod commands;
mod download;

use download::DownloadState;
use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            // Initialize download manager state
            let download_state = DownloadState::new();
            app.manage(Arc::new(Mutex::new(download_state)));

            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;

                // Register global shortcuts for DevTools
                // Cmd/Ctrl+J → open DevTools  |  Cmd/Ctrl+K → close DevTools
                use tauri_plugin_global_shortcut::GlobalShortcutExt;
                let gs = app.handle().global_shortcut();

                let handle_open = app.handle().clone();
                gs.on_shortcut("CmdOrCtrl+KeyJ", move |_app, _shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        if let Some(window) = handle_open.get_webview_window("main") {
                            let _ = window.open_devtools();
                        }
                    }
                })?;

                let handle_close = app.handle().clone();
                gs.on_shortcut("CmdOrCtrl+KeyK", move |_app, _shortcut, event| {
                    if event.state == tauri_plugin_global_shortcut::ShortcutState::Pressed {
                        if let Some(window) = handle_close.get_webview_window("main") {
                            let _ = window.close_devtools();
                        }
                    }
                })?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::download_image,
            commands::cancel_download,
            commands::get_downloads,
            commands::clear_download,
            commands::open_download_folder,
            commands::get_app_data_path,
            commands::fetch_wallpaper_image,
            commands::fetch_wallhaven_api,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
