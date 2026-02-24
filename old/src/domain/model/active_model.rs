use crate::domain::model::Entity;
use crate::domain::repositories::Repository;
use crate::domain::TikalResult;

pub trait ActiveModel: Entity {
    fn save<R>(self, repo: &R) -> impl std::future::Future<Output = TikalResult<Self>> + Send
    where
        R: Repository<Self>,
        Self: Sized,
    {
        async move {
            repo.save(&self).await?;
            Ok(self)
        }
    }

    fn is_new(&self) -> bool {
        let values = self.to_values();
        let pk = Self::primary_key();
        !values.contains_key(pk) || matches!(values[pk], crate::domain::value_objects::Value::Null)
    }
}

pub trait NewEntity: Entity + Default {
    fn new() -> Self {
        Self::default()
    }
}

impl<E> ActiveModel for E where E: Entity {}
