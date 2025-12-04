use std::sync::Arc;

use crate::trick_models::{Trick, TrickCreateInput, TrickError, TrickReplaceInput};
use crate::trick_repository::TrickRepository;
use uuid::Uuid;

pub struct TrickService {
    trick_repository: Arc<TrickRepository>,
}

impl TrickService {
    pub fn new(trick_repository: Arc<TrickRepository>) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trick_models::{TrickCreateInput, TrickError};
    use std::sync::Arc;

    mod create {
        use super::*;

        #[tokio::test]
        async fn should_create_a_trick() {
            let trick_repository = Arc::new(TrickRepository::new());
            let trick_service = TrickService::new(trick_repository.clone());

            let input = TrickCreateInput {
                title: "Sit".to_string(),
                description: "Sit...".to_string(),
                instructions: vec![],
            };

            // Act: call the service method
            let created_trick = trick_service.create(input).await.unwrap();

            // Assert: returned value contains the expected data
            assert_eq!(created_trick.title, "Sit");
            assert_eq!(created_trick.description, "Sit...");
            assert!(created_trick.instructions.is_empty());

            // Assert: repository contains the persisted trick
            let all_tricks = trick_repository.find_all().await;
            assert_eq!(all_tricks.len(), 1);
            assert_eq!(all_tricks[0].title, "Sit");
            assert_eq!(all_tricks[0].description, "Sit...");
        }

        #[tokio::test]
        async fn should_fail_when_title_is_empty() {
            let trick_repository = Arc::new(TrickRepository::new());
            let trick_service = TrickService::new(trick_repository.clone());

            let input = TrickCreateInput {
                title: "".to_string(),
                description: "".to_string(),
                instructions: vec![],
            };

            // Act: call the service method with invalid data
            let result = trick_service.create(input).await;

            // Assert: the call must fail
            assert!(result.is_err());

            // Assert: the error is the expected validation error
            match result.err().unwrap() {
                TrickError::Validation(msg) => {
                    assert_eq!(msg, "title must not be empty");
                }
                other => panic!("expected validation error, got: {:?}", other),
            }

            // Assert: repository should not contain any data
            let all_tricks = trick_repository.find_all().await;
            assert_eq!(all_tricks.len(), 0);
        }
    }
}
