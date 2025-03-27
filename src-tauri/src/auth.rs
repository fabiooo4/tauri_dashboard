use crate::database::{User, UserDb, UserDbError};

#[tauri::command]
pub fn login(username: &str, password: &str) -> String {
    format!("Hello, {}!\nYour password is: {}", username, password)
}

#[tauri::command]
pub fn register(username: &str, password: &str) -> Result<(), UserDbError> {
    UserDb::push(User::new(username, password))?;

    Ok(())
}
