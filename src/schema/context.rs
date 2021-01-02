use std::sync::Arc;

use crate::{recipe::Recipe, review::Reviews, watcher::CollectionWatcher};

pub struct Context {
    pub recipes: Arc<CollectionWatcher<Recipe>>,
    pub reviews: Arc<CollectionWatcher<Reviews>>,
}
