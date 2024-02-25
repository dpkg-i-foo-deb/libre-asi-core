pub mod connection;
pub mod question;
pub mod schema;

use std::{error::Error, fmt::Debug};

#[derive(thiserror::Error)]
pub enum SqliteError {
    #[error("Something went wrong")]
    GenericError(),
    #[error("Connection error")]
    ConnectionError(),
    #[error("Entity not found")]
    EntityNotFound(),
}

impl Debug for SqliteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}

impl From<sqlx::Error> for SqliteError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            _ => {
                eprintln!("{error}");

                Self::GenericError()
            }
        }
    }
}
