use tauri::State;

use crate::database::{User, UserDb, UserDbError};

#[tauri::command]
pub fn login(
    username: &str,
    password: &str,
    user_db: State<'_, UserDb>,
) -> Result<User, UserDbError> {
    let user = User::new(username, password);

    if user.is_empty() {
        return Err(UserDbError::EmptyUser);
    }

    match user_db.contains(&user) {
        true => Ok(user),
        false => Err(UserDbError::NoUserFound),
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
