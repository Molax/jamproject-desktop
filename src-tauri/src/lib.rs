mod discord_rpc;

use tauri::Manager;

#[tauri::command]
fn update_presence(
    state: tauri::State<'_, discord_rpc::PresenceState>,
    payload: discord_rpc::PresencePayload,
) -> Result<(), String> {
    discord_rpc::set_activity(&state, payload).map_err(|e| e.to_string())
}

#[tauri::command]
fn clear_presence(state: tauri::State<'_, discord_rpc::PresenceState>) -> Result<(), String> {
    discord_rpc::clear_activity(&state).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(discord_rpc::PresenceState::new());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![update_presence, clear_presence])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
