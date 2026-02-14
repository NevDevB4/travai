use tauri::{GlobalShortcutManager, Manager};

pub fn register_hotkeys(app: &tauri::AppHandle) -> Result<(), String> {
    let app_handle = app.clone();
    
    let mut shortcut_manager = app.global_shortcut_manager();

    // Register Ctrl+Shift+T to toggle Travai window
    shortcut_manager
        .register("ctrl+shift+t", move || {
            println!("[Hotkey] Ctrl+Shift+T pressed");
            
            if let Some(window) = app_handle.get_window("main") {
                match window.is_visible() {
                    Ok(is_visible) => {
                        if is_visible {
                            println!("[Hotkey] Hiding window");
                            let _ = window.hide();
                        } else {
                            println!("[Hotkey] Showing window");
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    Err(e) => {
                        eprintln!("[Hotkey] Error checking visibility: {}", e);
                    }
                }
            } else {
                eprintln!("[Hotkey] Main window not found");
            }
        })
        .map_err(|e| {
            let error_msg = format!("Failed to register hotkey: {}", e);
            eprintln!("[Hotkey] {}", error_msg);
            error_msg
        })?;

    println!("[Hotkey] Successfully registered Ctrl+Shift+T");
    Ok(())
}

pub fn unregister_hotkeys(app: &tauri::AppHandle) -> Result<(), String> {
    let mut shortcut_manager = app.global_shortcut_manager();
    shortcut_manager
        .unregister("ctrl+shift+t")
        .map_err(|e| format!("Failed to unregister hotkey: {}", e))?;
    Ok(())
}
