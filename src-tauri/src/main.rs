// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray};

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}
fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");

    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                // Changing System tray
                let tray_handle = event.window().app_handle().tray_handle();
                let item_handle = tray_handle.get_item("hide");
                item_handle.set_title("Show").expect("Failed to set title");
                api.prevent_close();
            }
            tauri::WindowEvent::Focused(true) => {
                println!("Window focused");
                let tray_handle = event.window().app_handle().tray_handle();
                let item_handle = tray_handle.get_item("hide");
                item_handle.set_title("Hide").expect("Failed to set title");
            }
            _ => {}
        })
        .system_tray(tray)
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .on_system_tray_event(|app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => {
                let _item_handle = app.tray_handle().get_item(&id);
                match id.as_str() {
                    "quit" => {
                        app.exit(0);
                    }
                    "hide" => {
                        let window = app.get_window("main").expect("Failed to get window");
                        let window_visibility = window
                            .is_visible()
                            .expect("Failed to get window visibility");

                        if window_visibility == true {
                            window.hide().expect("Failed to hide window");
                            _item_handle.set_title("Show").expect("Failed to set title")
                        } else {
                            window.show().expect("Failed to show window");
                            _item_handle.set_title("Hide").expect("Failed to set title")
                        }
                    }

                    _ => {}
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
