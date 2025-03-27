use std::{
    fs::{self, File},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

// Database config
static USER_DB_PATH: &str = "database/users.csv";

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct User {
    username: String,
    password: String,
}

impl User {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

pub struct UserDb;
impl UserDb {
    /// Get the file descriptor of the database csv file
    fn get_db(path: &Path) -> File {
        if let Ok(exists) = path.try_exists() {
            if !exists {
                if let Some(dir) = path.parent() {
                    fs::create_dir_all(dir)
                        .unwrap_or_else(|_| panic!("Failed to create directory: {:?}", dir))
                }

                File::create(path).expect("Can't open file in read-write mode");
            }

            let mut db = File::options()
                .read(true)
                .append(true)
                .open(path)
                .expect("Can't open file in read-write mode");

            // If file is empty insert a csv header by serializing the User struct
            if db.metadata().expect("Unable to access file").len() == 0 {
                let mut wtr = csv::Writer::from_writer(vec![]);

                wtr.serialize(User::new("", "")).unwrap();
                let content = wtr.into_inner().unwrap();

                // remove the last line to keep only the header
                db.write_all(&content[..content.len() - 2])
                    .unwrap_or_else(|_| panic!("Unable to write to file: {:?}", path));
            }

            db
        } else {
            panic!("Cannot check existance of file {path:?}");
        }
    }

    /// Check if a user is already registered
    pub fn contains(user: &User) -> bool {
        let db = UserDb::get_db(Path::new(USER_DB_PATH));
        let mut reader = csv::Reader::from_reader(db);

        reader
            .deserialize::<User>()
            .map(|u| u.expect("Failed to deserialize User"))
            .any(|u| u == *user)
    }

    /// Add a user entry to the database
    pub fn push(new_user: User) -> Result<(), UserDbError> {
        if UserDb::contains(&new_user) {
            return Err(UserDbError::ExistingUser);
        } else {
            let db = UserDb::get_db(Path::new(USER_DB_PATH));
            let mut wtr = csv::WriterBuilder::new().has_headers(false).from_writer(db);

            wtr.serialize(new_user)?;
        }

        Ok(())
    }
}

// Custom serializable error
#[derive(Debug, thiserror::Error)]
pub enum UserDbError {
    #[error(transparent)]
    Csv(#[from] csv::Error),
    #[error("The user is already registered")]
    ExistingUser,
}

impl Serialize for UserDbError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
