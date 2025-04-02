use std::{
    fs::{self, File},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    pub username: String,
    password: String,
    pub is_admin: bool,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: String::from(username),
            password: String::from(password),
            is_admin: username == "admin" && password == "admin",
        }
    }

    pub fn is_empty(&self) -> bool {
        self.username.is_empty() || self.password.is_empty()
    }
}

pub struct UserDb<'a> {
    path: &'a Path,
    current_user: Option<User>,
}

impl<'a> UserDb<'a> {
    pub fn new(path: &'a str) -> Self {
        Self {
            path: Path::new(path),
            current_user: None,
        }
    }

    pub fn set_current_user(&mut self, user: Option<User>) {
        self.current_user = user;
    }

    pub fn get_current_user(&self) -> Option<User> {
        self.current_user.clone()
    }

    /// Get the file descriptor of the database csv file
    fn get_db(&self) -> File {
        if let Ok(exists) = self.path.try_exists() {
            if !exists {
                if let Some(dir) = self.path.parent() {
                    fs::create_dir_all(dir)
                        .unwrap_or_else(|_| panic!("Failed to create directory: {:?}", dir))
                }

                File::create(self.path).expect("Can't open file in read-write mode");
            }

            let db = File::options()
                .read(true)
                .write(true)
                .open(self.path)
                .expect("Can't open file in read-write mode");

            db
        } else {
            panic!("Cannot check existance of file {:?}", self.path);
        }
    }

    /// Check if a user is already registered
    pub fn contains(&self, user: &User) -> bool {
        let mut reader = csv::Reader::from_reader(self.get_db());

        reader
            .deserialize::<User>()
            .filter_map(|result| result.ok())
            .any(|u| u == *user)
    }

    /// Add a user entry to the database
    pub fn push(&self, new_user: User) -> Result<(), UserDbError> {
        if new_user.is_empty() {
            return Err(UserDbError::EmptyUser);
        }

        if self.contains(&new_user) {
            return Err(UserDbError::ExistingUser);
        }

        let mut reader = csv::Reader::from_reader(self.get_db());
        let mut writer = csv::Writer::from_writer(self.get_db());

        for result in reader.deserialize::<User>() {
            let user = result?;

            writer.serialize(user)?;
        }

        writer.serialize(new_user)?;
        writer.flush()?;

        Ok(())
    }

    pub fn edit(&self, old_value: &User, new_value: User) -> Result<(), UserDbError> {
        if !self.contains(old_value) {
            return Err(UserDbError::NoUserFound);
        }

        let mut reader = csv::Reader::from_reader(self.get_db());
        let mut writer = csv::Writer::from_writer(self.get_db());

        for result in reader.deserialize::<User>() {
            let mut user = result?;

            // Edit the record old_value with the new_value
            if user == *old_value {
                user = new_value.clone();
            }

            writer.serialize(user)?;
        }

        writer.flush()?;

        Ok(())
    }

    pub fn get_users(&self) -> Vec<User> {
        let mut reader = csv::Reader::from_reader(self.get_db());

        reader.deserialize::<User>().map(|u| u.unwrap()).collect()
    }
}

// Custom serializable error
#[derive(Debug, thiserror::Error)]
pub enum UserDbError {
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("The user is already registered")]
    ExistingUser,
    #[error("No username or password provided")]
    EmptyUser,
    #[error("Wrong username or password")]
    NoUserFound,
}

impl Serialize for UserDbError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
