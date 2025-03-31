use std::sync::Mutex;

use tauri::State;

use crate::database::{User, UserDb, UserDbError};

#[tauri::command]
pub fn login(
    username: &str,
    password: &str,
    user_db: State<Mutex<UserDb>>,
) -> Result<User, UserDbError> {
    let mut user_db = user_db.lock().unwrap();
    let user = User::new(username, password);

    if user.is_empty() {
        return Err(UserDbError::EmptyUser);
    }

    match user_db.contains(&user) {
        true => {
            user_db.set_current_user(Some(user.clone()));
            Ok(user)
        },
        false => Err(UserDbError::NoUserFound),
    }
}

#[tauri::command]
pub fn logout(
    user_db: State<Mutex<UserDb>>,
) {
    let mut user_db = user_db.lock().unwrap();
    user_db.set_current_user(None);
}

#[tauri::command]
pub fn register(
    username: &str,
    password: &str,
    user_db: State<Mutex<UserDb>>,
) -> Result<(), UserDbError> {
    let user_db = user_db.lock().unwrap();
    user_db.push(User::new(username, password))?;

    Ok(())
}

#[tauri::command]
pub fn get_current_user(user_db: State<Mutex<UserDb>>) -> Option<User> {
    let user_db = user_db.lock().unwrap();
    user_db.get_current_user()
}

