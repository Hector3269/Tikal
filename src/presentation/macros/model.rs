#[macro_export]
macro_rules! model {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        use crate::domain::{Entity, Identifiable, ActiveRecord, Model, DomainError, Timestamps, SoftDeletes};
        use crate::kernel::types::{db::DbRow, core::Value};
        use std::collections::HashMap;

        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $field: $ty,)*
            pub created_at: Option<chrono::DateTime<chrono::Utc>>,
            pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
            pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
        }

        impl Entity for $name {
            fn entity_name() -> &'static str {
                stringify!($name)
            }
        }

        impl Identifiable for $name {
            fn id(&self) -> &String {
                &self.id
            }
        }


        impl From<DbRow> for $name {
            fn from(row: DbRow) -> Self {
                Self {
                    $(
                        $field: match row.get(stringify!($field)) {
                            Some(Value::String(s)) => s.clone(),
                            Some(Value::Int(i)) => i.to_string(),
                            _ => "".to_string(),
                        },
                    )*
                    created_at: match row.get("created_at") {
                        Some(Value::String(s)) => chrono::DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&chrono::Utc)),
                        _ => None,
                    },
                    updated_at: match row.get("updated_at") {
                        Some(Value::String(s)) => chrono::DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&chrono::Utc)),
                        _ => None,
                    },
                    deleted_at: match row.get("deleted_at") {
                        Some(Value::String(s)) => chrono::DateTime::parse_from_rfc3339(s).ok().map(|dt| dt.with_timezone(&chrono::Utc)),
                        _ => None,
                    },
                }
            }
        }

        #[async_trait::async_trait]
        impl ActiveRecord for $name {
            fn table_name() -> String {
                stringify!($name).to_lowercase()
            }

            fn primary_key() -> String {
                "id".to_string()
            }

            fn from_row(row: DbRow) -> Result<Self, crate::kernel::error::KernelError> {
                Ok(Self::from(row))
            }

            fn attributes(&self) -> HashMap<String, Value> {
                let mut attrs = HashMap::new();
                $(
                    attrs.insert(stringify!($field).to_string(), Value::String(self.$field.clone()));
                )*
                if let Some(created_at) = self.created_at {
                    attrs.insert("created_at".to_string(), Value::String(created_at.to_rfc3339()));
                }
                if let Some(updated_at) = self.updated_at {
                    attrs.insert("updated_at".to_string(), Value::String(updated_at.to_rfc3339()));
                }
                if let Some(deleted_at) = self.deleted_at {
                    attrs.insert("deleted_at".to_string(), Value::String(deleted_at.to_rfc3339()));
                }
                attrs
            }

            async fn find<E: crate::infrastructure::database::executor::QueryExecutor + Send + Sync>(executor: &E, id: &String) -> Result<Option<Self>, DomainError> {
                let sql = format!("SELECT * FROM {} WHERE {} = ?", Self::table_name(), Self::primary_key());
                let params = vec![Value::String(id.clone())];
                let rows = match executor.query_raw(&sql, &params).await {
                    Ok(r) => r,
                    Err(e) => return Err(DomainError::Infrastructure { message: format!("Query error: {}", e) }),
                };
                Ok(rows.into_iter().next().map(Self::from))
            }

            async fn save<E: crate::infrastructure::database::executor::QueryExecutor + Send + Sync>(&mut self, executor: &E) -> Result<(), DomainError> {
                if Self::uses_timestamps() {
                    let now = Self::fresh_timestamp();
                    if self.id().is_empty() || self.id() == "0" {
                        self.set_created_at(now);
                    }
                    self.set_updated_at(now);
                }

                let attrs = self.attributes();
                let is_new = self.id().is_empty() || self.id() == "0";
                if is_new {
                    let columns: Vec<String> = attrs.keys().cloned().collect();
                    let placeholders: Vec<String> = (0..columns.len()).map(|_| "?".to_string()).collect();
                    let sql = format!("INSERT INTO {} ({}) VALUES ({})",
                        Self::table_name(),
                        columns.join(", "),
                        placeholders.join(", "));
                    let values: Vec<Value> = columns.iter().map(|c| attrs[c].clone()).collect();
                    match executor.execute_raw(&sql, &values).await {
                        Ok(_) => {},
                        Err(e) => return Err(DomainError::Infrastructure { message: format!("Insert error: {}", e) }),
                    }
                } else {
                    let set_clause: Vec<String> = attrs.keys().filter(|k| *k != "id").map(|k| format!("{} = ?", k)).collect();
                    let sql = format!("UPDATE {} SET {} WHERE {} = ?",
                        Self::table_name(),
                        set_clause.join(", "),
                        Self::primary_key());
                    let mut values: Vec<Value> = attrs.values().filter(|v| match v {
                        Value::String(s) => s != "id",
                        _ => true,
                    }).cloned().collect();
                    values.push(Value::String(self.id().clone()));
                    match executor.execute_raw(&sql, &values).await {
                        Ok(_) => {},
                        Err(e) => return Err(DomainError::Infrastructure { message: format!("Update error: {}", e) }),
                    }
                }
                Ok(())
            }

            async fn delete<E: crate::infrastructure::database::executor::QueryExecutor + Send + Sync>(&self, executor: &E) -> Result<(), DomainError> {
                if Self::uses_soft_deletes() {
                    let sql = format!("UPDATE {} SET deleted_at = ?, updated_at = ? WHERE {} = ?",
                        Self::table_name(), Self::primary_key());
                    let now = Self::fresh_timestamp().to_rfc3339();
                    let params = vec![
                        Value::String(now.clone()),
                        Value::String(now),
                        Value::String(self.id().clone())
                    ];
                    match executor.execute_raw(&sql, &params).await {
                        Ok(_) => {},
                        Err(e) => return Err(DomainError::Infrastructure { message: format!("Soft delete error: {}", e) }),
                    }
                } else {
                    // Hard delete
                    let sql = format!("DELETE FROM {} WHERE {} = ?", Self::table_name(), Self::primary_key());
                    let params = vec![Value::String(self.id().clone())];
                    match executor.execute_raw(&sql, &params).await {
                        Ok(_) => {},
                        Err(e) => return Err(DomainError::Infrastructure { message: format!("Delete error: {}", e) }),
                    }
                }
                Ok(())
            }
        }

        impl Timestamps for $name {
            fn get_created_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
                &self.created_at
            }

            fn get_updated_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
                &self.updated_at
            }

            fn set_created_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
                self.created_at = Some(timestamp);
            }

            fn set_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
                self.updated_at = Some(timestamp);
            }
        }

        impl SoftDeletes for $name {
            fn get_deleted_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
                &self.deleted_at
            }

            fn set_deleted_at(&mut self, timestamp: Option<chrono::DateTime<chrono::Utc>>) {
                self.deleted_at = timestamp;
            }
        }

        impl Model for $name {}
    };
}