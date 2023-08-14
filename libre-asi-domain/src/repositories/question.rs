use crate::models::{self, question::Question};

use super::RepositoryError;

pub trait QuestionRepository {
    fn get(&self, id: &str) -> Result<&models::question::Question, super::RepositoryError>;
    fn get_all(&self) -> Result<&Vec<models::question::Question>, super::RepositoryError>;
    fn save(&mut self, question: Question) -> Result<(), super::RepositoryError>;
}

pub struct InMemoryRepository {
    questions: Vec<Question>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let questions: Vec<Question> = vec![];
        Self { questions }
    }
}

impl QuestionRepository for InMemoryRepository {
    fn get(&self, id: &str) -> Result<&models::question::Question, super::RepositoryError> {
        match self.questions.iter().find(|&q| q.id() == id) {
            Some(q) => Ok(q),
            None => Err(RepositoryError::EntityNotFound()),
        }
    }

    fn get_all(&self) -> Result<&Vec<models::question::Question>, super::RepositoryError> {
        Ok(&self.questions)
    }

    fn save(&mut self, question: Question) -> Result<(), super::RepositoryError> {
        self.questions.push(question);

        Ok(())
    }
}
