use crate::trick_models::{Trick, TrickCreateInput, TrickReplaceInput};
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct TrickRepository {
    store: RwLock<HashMap<Uuid, Trick>>,
}

impl TrickRepository {
    pub fn new() -> TrickRepository {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }

    pub async fn create(&self, input: TrickCreateInput) -> Trick {
        let new_trick = Trick {
            id: Uuid::new_v4(),
            title: input.title,
            description: input.description,
            instructions: input.instructions,
        };

        let mut store = self.store.write().await;
        store.insert(new_trick.id.clone(), new_trick.clone());

        new_trick
    }

    pub async fn replace(&self, id: Uuid, input: TrickReplaceInput) -> Trick {
        let trick_to_replace = Trick {
            id: id.clone(),
            title: input.title,
            description: input.description,
            instructions: input.instructions,
        };

        let mut store = self.store.write().await;
        store.insert(id, trick_to_replace.clone());

        trick_to_replace
    }

    pub async fn find_all(&self) -> Vec<Trick> {
        let store = self.store.read().await;
        store.values().cloned().collect()
    }

    pub async fn find_by_id(&self, id: Uuid) -> Option<Trick> {
        let store = self.store.read().await;
        store.get(&id).cloned()
    }

    pub async fn delete_by_id(&self, id: Uuid) {
        let mut store = self.store.write().await;
        store.remove(&id);
    }
}
