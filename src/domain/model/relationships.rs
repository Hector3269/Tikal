use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum RelationshipType {
    BelongsTo,
    HasMany,
    HasOne,
    ManyToMany,
}

#[derive(Debug, Clone)]
pub struct RelationshipMeta {
    pub name: String,
    pub rel_type: RelationshipType,
    pub foreign_key: String,
    pub target_table: String,
    pub join_table: Option<String>,
    pub target_foreign_key: Option<String>,
    pub cascade_delete: bool,
    pub eager_load: bool,
}

impl RelationshipMeta {
    pub fn belongs_to(name: String, foreign_key: String, target_table: String) -> Self {
        Self {
            name,
            rel_type: RelationshipType::BelongsTo,
            foreign_key,
            target_table,
            join_table: None,
            target_foreign_key: None,
            cascade_delete: false,
            eager_load: false,
        }
    }

    pub fn has_many(name: String, foreign_key: String, target_table: String) -> Self {
        Self {
            name,
            rel_type: RelationshipType::HasMany,
            foreign_key,
            target_table,
            join_table: None,
            target_foreign_key: None,
            cascade_delete: false,
            eager_load: false,
        }
    }

    pub fn has_one(name: String, foreign_key: String, target_table: String) -> Self {
        Self {
            name,
            rel_type: RelationshipType::HasOne,
            foreign_key,
            target_table,
            join_table: None,
            target_foreign_key: None,
            cascade_delete: false,
            eager_load: false,
        }
    }

    pub fn many_to_many(
        name: String,
        join_table: String,
        foreign_key: String,
        target_foreign_key: String,
        target_table: String,
    ) -> Self {
        Self {
            name,
            rel_type: RelationshipType::ManyToMany,
            foreign_key,
            target_table,
            join_table: Some(join_table),
            target_foreign_key: Some(target_foreign_key),
            cascade_delete: false,
            eager_load: false,
        }
    }

    pub fn eager_load(mut self) -> Self {
        self.eager_load = true;
        self
    }

    pub fn cascade_delete(mut self) -> Self {
        self.cascade_delete = true;
        self
    }
}

pub type RelationshipMap = HashMap<String, RelationshipMeta>;

pub trait RelationshipHandler {
    fn get_relationships() -> &'static RelationshipMap;

    fn has_relationship(name: &str) -> bool {
        Self::get_relationships().contains_key(name)
    }

    fn get_relationship(name: &str) -> Option<&'static RelationshipMeta> {
        Self::get_relationships().get(name)
    }

    fn get_eager_relationships() -> Vec<&'static RelationshipMeta> {
        Self::get_relationships()
            .values()
            .filter(|rel| rel.eager_load)
            .collect()
    }

    fn generate_join_sql(
        base_table: &str,
        relationship: &RelationshipMeta,
    ) -> Result<String, String> {
        match relationship.rel_type {
            RelationshipType::BelongsTo => Ok(format!(
                "LEFT JOIN {} ON {}.{} = {}.{}",
                relationship.target_table,
                base_table,
                relationship.foreign_key,
                relationship.target_table,
                "id"
            )),
            RelationshipType::HasMany => Ok(format!(
                "LEFT JOIN {} ON {}.{} = {}.{}",
                relationship.target_table,
                relationship.target_table,
                relationship.foreign_key,
                base_table,
                "id"
            )),
            RelationshipType::HasOne => Ok(format!(
                "LEFT JOIN {} ON {}.{} = {}.{}",
                relationship.target_table,
                relationship.target_table,
                relationship.foreign_key,
                base_table,
                "id"
            )),
            RelationshipType::ManyToMany => {
                let join_table = relationship
                    .join_table
                    .as_ref()
                    .ok_or("Join table required for many-to-many")?;
                let target_fk = relationship
                    .target_foreign_key
                    .as_ref()
                    .ok_or("Target foreign key required for many-to-many")?;
                Ok(format!(
                    "LEFT JOIN {} ON {}.{} = {}.{} LEFT JOIN {} ON {}.{} = {}.{}",
                    join_table,
                    join_table,
                    relationship.foreign_key,
                    base_table,
                    "id",
                    relationship.target_table,
                    relationship.target_table,
                    "id",
                    join_table,
                    target_fk
                ))
            }
        }
    }

    fn generate_relationship_where(
        _base_table: &str,
        relationship: &RelationshipMeta,
        operator: &str,
        value: &str,
    ) -> Result<String, String> {
        match relationship.rel_type {
            RelationshipType::BelongsTo => Ok(format!(
                "{}.{} {} {}",
                relationship.target_table, "id", operator, value
            )),
            RelationshipType::HasMany | RelationshipType::HasOne => Ok(format!(
                "{}.{} {} {}",
                relationship.target_table, relationship.foreign_key, operator, value
            )),
            RelationshipType::ManyToMany => {
                let join_table = relationship
                    .join_table
                    .as_ref()
                    .ok_or("Join table required for many-to-many")?;
                Ok(format!(
                    "{}.{} {} {}",
                    join_table,
                    relationship
                        .target_foreign_key
                        .as_ref()
                        .ok_or("Target foreign key required for many-to-many")?,
                    operator,
                    value
                ))
            }
        }
    }
}

pub struct RelationshipQueryBuilder {
    joins: Vec<String>,
    wheres: Vec<String>,
    selects: Vec<String>,
}

impl RelationshipQueryBuilder {
    pub fn new() -> Self {
        Self {
            joins: Vec::new(),
            wheres: Vec::new(),
            selects: Vec::new(),
        }
    }

    pub fn add_join(mut self, join_sql: String) -> Self {
        self.joins.push(join_sql);
        self
    }

    pub fn add_where(mut self, where_sql: String) -> Self {
        self.wheres.push(where_sql);
        self
    }

    pub fn add_select(mut self, select_sql: String) -> Self {
        self.selects.push(select_sql);
        self
    }

    pub fn build(self, base_select: &str) -> String {
        let mut sql = base_select.to_string();

        if !self.joins.is_empty() {
            sql.push_str(" ");
            sql.push_str(&self.joins.join(" "));
        }

        if !self.wheres.is_empty() {
            if !sql.contains("WHERE") {
                sql.push_str(" WHERE ");
            } else {
                sql.push_str(" AND ");
            }
            sql.push_str(&self.wheres.join(" AND "));
        }

        sql
    }
}

impl Default for RelationshipQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

pub mod utils {
    use super::*;

    pub fn extract_foreign_key(
        values: &HashMap<String, crate::domain::value_objects::Value>,
        foreign_key: &str,
    ) -> Option<crate::domain::value_objects::Value> {
        values.get(foreign_key).cloned()
    }

    pub fn build_relationship_map(relationships: Vec<RelationshipMeta>) -> RelationshipMap {
        let mut map = HashMap::new();
        for rel in relationships {
            map.insert(rel.name.clone(), rel);
        }
        map
    }

    pub fn validate_relationship(relationship: &RelationshipMeta) -> Result<(), String> {
        match relationship.rel_type {
            RelationshipType::ManyToMany => {
                if relationship.join_table.is_none() {
                    return Err("Join table is required for many-to-many relationships".to_string());
                }
                if relationship.target_foreign_key.is_none() {
                    return Err(
                        "Target foreign key is required for many-to-many relationships".to_string(),
                    );
                }
            }
            _ => {}
        }

        if relationship.name.is_empty() {
            return Err("Relationship name cannot be empty".to_string());
        }

        if relationship.target_table.is_empty() {
            return Err("Target table cannot be empty".to_string());
        }

        Ok(())
    }
}
