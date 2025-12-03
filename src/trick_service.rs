use crate::trick_models::{Trick, TrickCreateInput, TrickError, TrickReplaceInput};
use crate::trick_repository::TrickRepository;
use uuid::Uuid;

pub struct TrickService {
    trick_repository: TrickRepository,
}

impl TrickService {
    pub fn new(trick_repository: TrickRepository) -> Self {
        Self { trick_repository }
    }

    pub async fn create(&self, input: TrickCreateInput) -> Result<Trick, TrickError> {
        self.validate_create(&input)?;
        Ok(self.trick_repository.create(input).await)
    }

    pub async fn replace(&self, id: Uuid, input: TrickReplaceInput) -> Result<Trick, TrickError> {
        self.validate_replace(id, &input).await?;
        Ok(self.trick_repository.replace(id, input).await)
    }

    pub async fn find_all(&self) -> Vec<Trick> {
        self.trick_repository.find_all().await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Trick, TrickError> {
        self.trick_repository
            .find_by_id(id)
            .await
            .ok_or(TrickError::NotFound(id))
    }

    pub async fn delete_by_id(&self, id: Uuid) {
        self.trick_repository.delete_by_id(id).await
    }

    fn validate_create(&self, data: &TrickCreateInput) -> Result<(), TrickError> {
        if data.title.trim().is_empty() {
            return Err(TrickError::Validation(
                "title must not be empty".to_string(),
            ));
        }

        Ok(())
    }

    async fn validate_replace(&self, id: Uuid, data: &TrickReplaceInput) -> Result<(), TrickError> {
        self.find_by_id(id).await?;

        if data.title.trim().is_empty() {
            return Err(TrickError::Validation(
                "Title must not be empty".to_string(),
            ));
        }

        Ok(())
    }
}
