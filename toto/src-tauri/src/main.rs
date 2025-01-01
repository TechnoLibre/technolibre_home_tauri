use tauri::{CustomMenuItem, SystemTray, SystemTrayMenu, SystemTrayMenuItem, WindowBuilder};
use leptos::*;
use leptos_dom::*;

fn main() {
    tauri::Builder::default()
        .setup(|_app| {
            // Setup code for your app
            Ok(())
        })
        .system_tray(SystemTray::new().with_menu(
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("quit".to_string(), "Quit"))
        ))
        .on_system_tray_event(|_, event| {
            if let tauri::SystemTrayEvent::MenuItemClick { id, .. } = event {
                if id == "quit" {
                    std::process::exit(0);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}