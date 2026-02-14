#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod llm;
mod commands;
mod clipboard;
mod hotkey;
mod tray;
mod window_state;

use tauri::Manager;
use std::sync::Mutex;

fn main() {
    println!("[Travai] Starting application...");
    
    tauri::Builder::default()
        .manage(Mutex::new(db::Database::new()))
        .system_tray(tray::create_tray())
        .on_system_tray_event(tray::handle_tray_event)
        .invoke_handler(tauri::generate_handler![
            commands::send_message,
            commands::generate_code,
            commands::explain_code,
            commands::debug_code,
            commands::optimize_code,
            commands::upload_and_analyze_file,
            commands::get_conversations,
            commands::create_conversation,
            commands::get_messages,
            commands::check_lm_studio_connection,
            commands::get_clipboard,
            commands::set_clipboard,
        ])
        .setup(|app| {
            println!("[Travai] Setup phase starting");
            
            // Register global hotkey
            match hotkey::register_hotkeys(app.handle()) {
                Ok(_) => println!("[Travai] Hotkeys registered successfully"),
                Err(e) => eprintln!("[Travai] Failed to register hotkeys: {}", e),
            }

            println!("[Travai] Setup phase complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
