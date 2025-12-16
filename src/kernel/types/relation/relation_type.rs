/// Represents the type of relationship between entities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationType {
    HasOne,
    HasMany,
    BelongsTo,
    BelongsToMany,
}