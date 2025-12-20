use crate::kernel::types::schema::{
    TableName, ColumnName, IndexName,
    ColumnType, Nullable, DefaultValue
};

#[derive(Debug, Clone)]
pub struct Table {
    pub name: TableName,
    pub columns: Vec<Column>,
    pub indexes: Vec<Index>,
    pub foreign_keys: Vec<ForeignKey>,
}

impl Table {
    pub fn new(name: TableName) -> Self {
        Self {
            name,
            columns: Vec::new(),
            indexes: Vec::new(),
            foreign_keys: Vec::new(),
        }
    }
    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    pub fn add_index(&mut self, index: Index) {
        self.indexes.push(index);
    }
    pub fn add_foreign_key(&mut self, foreign_key: ForeignKey) {
        self.foreign_keys.push(foreign_key);
    }
}

#[derive(Debug, Clone)]
pub struct Column {
    pub name: ColumnName,
    pub column_type: ColumnType,
    pub nullable: Nullable,
    pub default_value: Option<DefaultValue>,
    pub is_primary_key: bool,
}

impl Column {
    pub fn new(name: ColumnName, column_type: ColumnType, nullable: Nullable) -> Self {
        Self {
            name,
            column_type,
            nullable,
            default_value: None,
            is_primary_key: false,
        }
    }
    pub fn with_default(mut self, default_value: DefaultValue) -> Self {
        self.default_value = Some(default_value);
        self
    }
    pub fn primary_key(mut self) -> Self {
        self.is_primary_key = true;
        self
    }
}
#[derive(Debug, Clone)]
pub struct Index {

    pub name: IndexName,
    pub columns: Vec<ColumnName>,
    pub unique: bool,
}

impl Index {
    pub fn new(name: IndexName, columns: Vec<ColumnName>, unique: bool) -> Self {
        Self {
            name,
            columns,
            unique,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ForeignKey {
    pub constraint_name: Option<String>,
    pub local_columns: Vec<ColumnName>,
    pub referenced_table: TableName,
    pub referenced_columns: Vec<ColumnName>,
}

impl ForeignKey {
    pub fn new(
        local_columns: Vec<ColumnName>,
        referenced_table: TableName,
        referenced_columns: Vec<ColumnName>,
    ) -> Self {
        Self {
            constraint_name: None,
            local_columns,
            referenced_table,
            referenced_columns,
        }
    }
    pub fn with_constraint_name(mut self, name: String) -> Self {
        self.constraint_name = Some(name);
        self
    }
}