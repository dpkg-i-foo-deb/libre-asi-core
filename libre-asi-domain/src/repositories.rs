pub mod question;

use std::{error::Error, fmt::Debug};

#[derive(thiserror::Error)]
pub enum RepositoryError {
    #[error("Entity not found")]
    EntityNotFound(),
}

impl Debug for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        if let Some(source) = self.source() {
            writeln!(f, "Caused by:\n\t{}", source)?;
        }
        Ok(())
    }
}
