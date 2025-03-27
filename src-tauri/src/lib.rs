mod auth;
mod database;

use database::UserDb;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(UserDb::new("database/users.csv"))
        .invoke_handler(tauri::generate_handler![auth::login, auth::register])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
