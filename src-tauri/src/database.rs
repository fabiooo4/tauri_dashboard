use std::{
    fs::{self, File},
    io::{BufRead, Write},
    path::Path,
    str::from_utf8,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct User {
    username: String,
    password: String,
    is_admin: bool,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
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

            let mut db = File::options()
                .read(true)
                .append(true)
                .open(self.path)
                .expect("Can't open file in read-write mode");

            // If file is empty insert a csv header by serializing the User struct
            if db.metadata().expect("Unable to access file").len() == 0 {
                let mut wtr = csv::Writer::from_writer(vec![]);

                wtr.serialize(User::new("", "")).unwrap();

                // remove the last line to keep only the header
                let content = wtr.into_inner().expect("Unable to serialize user");
                let header = from_utf8(&content)
                    .expect("Invalid UTF-8 sequence")
                    .lines()
                    .next()
                    .unwrap()
                    .to_owned()
                    + "\n";

                db.write_all(header.as_bytes())
                    .unwrap_or_else(|_| panic!("Unable to write to file: {:?}", self.path));
            }

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

        let mut wtr = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(self.get_db());

        wtr.serialize(new_user)?;

        wtr.flush()?;

        Ok(())
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
