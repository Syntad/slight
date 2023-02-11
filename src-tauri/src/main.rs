#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use spotlight::init_spotlight_window;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem,
};

#[allow(unused_imports)]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

mod spotlight;

fn create_system_tray() -> SystemTray {
    let quit = CustomMenuItem::new("Quit".to_string(), "Quit");
    let show = CustomMenuItem::new("Show".to_string(), "Show");
    let hide = CustomMenuItem::new("Hide".to_string(), "Hide");
    let preferences = CustomMenuItem::new("Preferences".to_string(), "Preferences");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_item(preferences)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    SystemTray::new().with_menu(tray_menu)
}

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        app.set_activation_policy(tauri::ActivationPolicy::Accessory);
        let window = app.get_window("main").unwrap();

        #[cfg(target_os = "macos")]
        apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, Some(10.0))
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

        init_spotlight_window(window);
        Ok(())
    })
    .system_tray(create_system_tray())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
