use crate::database::{User, UserDb, UserDbError};

#[tauri::command]
pub fn login(username: &str, password: &str) -> Option<User> {
    let user = User::new(username, password);
    match UserDb::contains(&user) {
        true => Some(user),
        false => None,
    }
}

#[tauri::command]
pub fn register(username: &str, password: &str) -> Result<(), UserDbError> {
    UserDb::push(User::new(username, password))?;

    Ok(())
}
