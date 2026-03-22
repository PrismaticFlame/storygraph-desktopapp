use tauri_plugin_autostart::ManagerExt;
use tauri::{
    Manager,
    tray::{MouseButton, TrayIconBuilder, TrayIconEvent},
    menu::{Menu, MenuItem},
    WindowEvent,
};

use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, Some(vec!["--hidden"])))
        .setup(|app| {
            let start_hidden = std::env::args().any(|a| a == "--hidden");
            if !start_hidden {
                if let Some(win) = app.get_webview_window("main") {
                    let _ = win.maximize();
                    let _ = win.show();
                }
            }

            let autostart_manager = app.autolaunch();
            let is_enabled = autostart_manager.is_enabled().unwrap_or(false);
            let autostart_label = if is_enabled {
                "Disable Launch at Startup"
            } else {
                "Enable Launch at Startup"
            };

            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let autostart = MenuItem::with_id(app, "autostart", autostart_label, true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &autostart, &quit])?;

            let autostart_item = autostart.clone();
            TrayIconBuilder::with_id("main")
                .menu(&menu)
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(move |app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(win) = app.get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                    "autostart" => {
                        let manager = app.autolaunch();
                        let enabled = manager.is_enabled().unwrap_or(false);
                        if enabled {
                            let _ = manager.disable();
                        } else {
                            let _ = manager.enable();
                        }
                        let now_enabled = manager.is_enabled().unwrap_or(false);
                        let new_label = if now_enabled { "Disable Launch at Startup" } else { "Enable Launch at Startup" };
                        let _ = autostart_item.set_text(new_label);
                    }
                    "quit" => std::process::exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, .. } = event {
                        if let Some(win) = tray.app_handle().get_webview_window("main") {
                            let _ = win.show();
                            let _ = win.set_focus();
                        }
                    }
                })
                .build(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}