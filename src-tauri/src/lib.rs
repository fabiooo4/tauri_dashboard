mod auth;
mod database;

use std::sync::Mutex;

use database::UserDb;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(UserDb::new("database/users.csv")))
        .invoke_handler(tauri::generate_handler![
            auth::login,
            auth::logout,
            auth::register,
            auth::get_current_user,
            auth::get_users,
            auth::toggle_admin,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
