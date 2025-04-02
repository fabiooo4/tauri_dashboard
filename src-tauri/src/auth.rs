use std::sync::Mutex;

use tauri::{AppHandle, Emitter, State};

use crate::database::{User, UserDb, UserDbError};

#[tauri::command]
pub fn login(
    username: &str,
    password: &str,
    user_db: State<Mutex<UserDb>>,
    app: AppHandle,
) -> Result<User, UserDbError> {
    let mut user_db = user_db
        .lock()
        .expect("Failed to obtain lock on user database");
    let user = User::new(username, password);

    if user.is_empty() {
        return Err(UserDbError::EmptyUser);
    }

    match user_db.contains(&user) {
        true => {
            user_db.set_current_user(Some(user.clone()));
            app.emit("logged-in", user.clone())
                .expect("Failed to emit logged-in event");
            Ok(user)
        }
        false => Err(UserDbError::NoUserFound),
    }
}

#[tauri::command]
pub fn logout(user_db: State<Mutex<UserDb>>, app: AppHandle) {
    let mut user_db = user_db
        .lock()
        .expect("Failed to obtain lock on user database");
    user_db.set_current_user(None);
    app.emit("logged-out", ())
        .expect("Failed to emit logged-out event");
}

#[tauri::command]
pub fn register(
    username: &str,
    password: &str,
    user_db: State<Mutex<UserDb>>,
) -> Result<(), UserDbError> {
    let user_db = user_db
        .lock()
        .expect("Failed to obtain lock on user database");
    user_db.push(User::new(username, password))?;

    Ok(())
}

#[tauri::command]
pub fn get_current_user(user_db: State<Mutex<UserDb>>) -> Option<User> {
    let user_db = user_db
        .lock()
        .expect("Failed to obtain lock on user database");
    user_db.get_current_user()
}

#[tauri::command]
pub fn get_users(user_db: State<Mutex<UserDb>>) -> Vec<User> {
    let user_db = user_db
        .lock()
        .expect("Failed to obtain lock on user database");
    user_db.get_users()
}

#[tauri::command]
pub fn toggle_admin(
    user: User,
    user_db: State<Mutex<UserDb>>,
    app: AppHandle,
) -> Result<User, UserDbError> {
    let user_db = user_db
        .lock()
        .expect("Failed to obtain lock on user database");
    let mut new_user = user.clone();
    new_user.is_admin = !new_user.is_admin;

    user_db.edit(&user, new_user.clone())?;

    app.emit("updated-users", user)
        .expect("Failed to emit event");

    Ok(new_user)
}
