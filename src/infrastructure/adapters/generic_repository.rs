use async_trait::async_trait;
use crate::infrastructure::database::executor::QueryExecutor;
use crate::domain::repositories::Repository;
use crate::kernel::error::KernelError;

pub struct GenericRepository<T, E> {
    _executor: E,
    _table_name: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, E> GenericRepository<T, E> {
    pub fn new(executor: E, table_name: String) -> Self {
        Self {
            _executor: executor,
            _table_name: table_name,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[async_trait(?Send)]
impl<T, E> Repository<T> for GenericRepository<T, E>
where
    T: crate::domain::Model + Send + Sync + 'static,
    E: QueryExecutor + Send + Sync,
{
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, KernelError> {
        let id_string = id.to_string();
        T::find(&self._executor, &id_string).await
    }

    async fn find_all(&self) -> Result<Vec<T>, KernelError> {
        T::all(&self._executor).await
    }

    async fn save(&self, mut entity: T) -> Result<T, KernelError> {
        entity.save(&self._executor).await?;
        Ok(entity)
    }

    async fn delete_by_id(&self, id: &str) -> Result<(), KernelError> {
        if let Some(entity) = self.find_by_id(id).await? {
            entity.delete(&self._executor).await
        } else {
            Ok(())
        }
    }

    async fn exists_by_id(&self, id: &str) -> Result<bool, KernelError> {
        Ok(self.find_by_id(id).await?.is_some())
    }
}