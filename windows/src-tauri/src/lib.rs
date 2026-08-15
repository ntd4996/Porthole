mod actions;
mod commands;
mod dto;
mod ignore_store;
mod project_win;
mod scan;

use commands::AppState;
use std::sync::Mutex;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager,
};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(AppState(Mutex::new(ignore_store::IgnoreStore::load())))
        .invoke_handler(tauri::generate_handler![
            commands::scan_ports,
            commands::ignore_process,
            commands::ignore_port,
            commands::unignore_process,
            commands::unignore_port,
            commands::unignore_matching,
            commands::kill,
            commands::open_url,
        ])
        .setup(|app| {
            let open_i = MenuItem::with_id(app, "open", "Open Porthole", true, None::<&str>)?;
            let refresh_i = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;
            let updates_i =
                MenuItem::with_id(app, "updates", "Check for Updates…", true, None::<&str>)?;
            let quit_i = MenuItem::with_id(app, "quit", "Quit Porthole", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&open_i, &refresh_i, &updates_i, &quit_i])?;

            let mut builder = TrayIconBuilder::with_id("main")
                .tooltip("Porthole")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "open" => toggle_window(app),
                    "refresh" => {
                        let _ = app.emit("porthole://refresh", ());
                    }
                    "updates" => {
                        let _ = app.emit("porthole://check-updates", ());
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_window(tray.app_handle());
                    }
                });
            if let Some(icon) = app.default_window_icon() {
                builder = builder.icon(icon.clone());
            }
            builder.build(app)?;

            // Hide the popover when it loses focus (click outside / switch app).
            if let Some(win) = app.get_webview_window("main") {
                let w = win.clone();
                win.on_window_event(move |event| {
                    if let tauri::WindowEvent::Focused(false) = event {
                        let _ = w.hide();
                    }
                });
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running Porthole");
}

fn toggle_window(app: &tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("main") {
        if win.is_visible().unwrap_or(false) {
            let _ = win.hide();
        } else {
            let _ = win.show();
            let _ = win.set_focus();
        }
    }
}
