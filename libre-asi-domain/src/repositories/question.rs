use crate::models;

pub trait QuestionRepository {
    fn get(&self, id: &str) -> Result<models::question::Question, super::RepositoryError>;
    fn get_all(&self) -> Result<Vec<models::question::Question>, super::RepositoryError>;
    fn save(&self) -> Result<(), super::RepositoryError>;
}
