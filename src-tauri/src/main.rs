#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use spotlight::init_spotlight_window;
use tauri::Manager;

#[allow(unused_imports)]
use window_vibrancy::{apply_vibrancy, NSVisualEffectMaterial};

mod spotlight;

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
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
