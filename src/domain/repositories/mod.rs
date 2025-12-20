use async_trait::async_trait;
use crate::domain::Entity;
use crate::kernel::error::KernelError;

#[async_trait(?Send)]
pub trait Repository<T: Entity + Send + Sync> {
    async fn find_by_id(&self, id: &str) -> Result<Option<T>, KernelError>;
    async fn find_all(&self) -> Result<Vec<T>, KernelError>;
    async fn save(&self, entity: T) -> Result<T, KernelError>;
    async fn delete_by_id(&self, id: &str) -> Result<(), KernelError>;
    async fn exists_by_id(&self, id: &str) -> Result<bool, KernelError>;
}