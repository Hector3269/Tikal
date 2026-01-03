use crate::domain::model::Entity;
use crate::domain::repositories::Repository;
use crate::domain::value_objects::Value;
use crate::domain::TikalResult;
use std::sync::Arc;

pub struct Lazy<T> {
    foreign_key: Value,
    loaded: std::sync::Mutex<Option<T>>,
    loader: Arc<
        dyn Fn(
                Value,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = TikalResult<Option<T>>> + Send>,
            > + Send
            + Sync,
    >,
}

impl<T> Lazy<T> {
    pub fn new<F, Fut>(foreign_key: Value, loader: F) -> Self
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = TikalResult<Option<T>>> + Send + 'static,
    {
        let loader = Arc::new(move |fk: Value| {
            Box::pin(loader(fk))
                as std::pin::Pin<
                    Box<dyn std::future::Future<Output = TikalResult<Option<T>>> + Send>,
                >
        });
        Self {
            foreign_key,
            loaded: std::sync::Mutex::new(None),
            loader,
        }
    }

    pub async fn load(&self) -> TikalResult<Option<T>>
    where
        T: Clone,
    {
        let mut loaded = self.loaded.lock().unwrap();
        if loaded.is_none() {
            *loaded = (self.loader)(self.foreign_key.clone()).await?;
        }
        Ok(loaded.clone())
    }

    pub fn get(&self) -> Option<T>
    where
        T: Clone,
    {
        self.loaded.lock().unwrap().clone()
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded.lock().unwrap().is_some()
    }

    pub fn set_loaded(&self, value: T) {
        *self.loaded.lock().unwrap() = Some(value);
    }
}

impl<T> Clone for Lazy<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            foreign_key: self.foreign_key.clone(),
            loaded: std::sync::Mutex::new(self.loaded.lock().unwrap().clone()),
            loader: self.loader.clone(),
        }
    }
}

pub fn belongs_to_lazy<R>(foreign_key: Value, repo: Arc<dyn Repository<R> + Send + Sync>) -> Lazy<R>
where
    R: Entity + Clone + Send + Sync + 'static,
{
    Lazy::new(foreign_key, move |fk| {
        let repo = Arc::clone(&repo);
        async move { repo.find_by_id(&fk).await }
    })
}

impl<T> std::fmt::Debug for Lazy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Lazy")
            .field("foreign_key", &self.foreign_key)
            .field("is_loaded", &self.is_loaded())
            .finish()
    }
}
