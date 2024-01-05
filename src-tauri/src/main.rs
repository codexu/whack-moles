// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, SystemTrayEvent, Manager};
mod scan;
use scan::{ scan_once, scan_loop, scan_colors, stop_scan };

fn main() {
    let option = CustomMenuItem::new("option".to_string(), "打开设置中心");
    let quit = CustomMenuItem::new("quit".to_string(), "退出");
    let tray_menu = SystemTrayMenu::new()
        .add_item(option)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "option" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                _ => {}
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![scan_once, scan_loop, scan_colors, stop_scan])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

