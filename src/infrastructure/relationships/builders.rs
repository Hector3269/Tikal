use crate::kernel::types::core::Value;
use crate::infrastructure::types::DbResult;
use crate::infrastructure::database::executor::QueryExecutor;

pub struct HasManyBuilder<T> {
    pub related_model: std::marker::PhantomData<T>,
    pub foreign_key: String,
    pub local_key: String,
}

impl<T> HasManyBuilder<T> {
    pub fn new(foreign_key: String, local_key: String) -> Self {
        Self {
            related_model: std::marker::PhantomData,
            foreign_key,
            local_key,
        }
    }

    pub async fn get<E>(&self, local_value: impl Into<Value>, executor: &E) -> DbResult<Vec<T>>
    where
        T: crate::domain::ActiveRecord + From<crate::kernel::types::db::DbRow>,
        E: QueryExecutor,
    {
        let sql = format!("SELECT * FROM {} WHERE {} = ?", T::table_name(), self.foreign_key);
        let params = vec![local_value.into()];
        let rows = executor.query_raw(&sql, &params).await?;
        Ok(rows.into_iter().map(T::from).collect())
    }
}

pub struct BelongsToBuilder<T> {
    pub related_model: std::marker::PhantomData<T>,
    pub foreign_key: String,
    pub owner_key: String,
}

impl<T> BelongsToBuilder<T> {
    pub fn new(foreign_key: String, owner_key: String) -> Self {
        Self {
            related_model: std::marker::PhantomData,
            foreign_key,
            owner_key,
        }
    }

    pub async fn get<E>(&self, foreign_value: impl Into<Value>, executor: &E) -> DbResult<Option<T>>
    where
        T: crate::domain::ActiveRecord + From<crate::kernel::types::db::DbRow>,
        E: QueryExecutor,
    {
        let sql = format!("SELECT * FROM {} WHERE {} = ?", T::table_name(), self.owner_key);
        let params = vec![foreign_value.into()];
        let rows = executor.query_raw(&sql, &params).await?;
        Ok(rows.into_iter().next().map(T::from))
    }
}

pub fn has_many<T>(_local_value: impl Into<Value>, foreign_key: &str) -> HasManyBuilder<T>
where
    T: crate::domain::Entity,
{
    HasManyBuilder::new(foreign_key.to_string(), "id".to_string())
}

pub fn belongs_to<T>(_foreign_value: impl Into<Value>, owner_key: &str) -> BelongsToBuilder<T>
where
    T: crate::domain::Entity,
{
    BelongsToBuilder::new("id".to_string(), owner_key.to_string())
}