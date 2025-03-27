use tauri::State;

use crate::database::{User, UserDb, UserDbError};

#[tauri::command]
pub fn login(username: &str, password: &str, user_db: State<'_, UserDb>) -> Option<User> {
    let user = User::new(username, password);
    match user_db.contains(&user) {
        true => Some(user),
        false => None,
    }
}

#[tauri::command]
pub fn register(
    username: &str,
    password: &str,
    user_db: State<'_, UserDb>,
) -> Result<(), UserDbError> {
    user_db.push(User::new(username, password))?;

    Ok(())
}
