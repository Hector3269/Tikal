use crate::domain::model::relationships::RelationshipType;
use crate::domain::model::Entity;
use crate::domain::query::builder as domain_builder;
use crate::infrastructure::query_builder::ast::*;

pub struct QueryAstBuilder;

impl QueryAstBuilder {
    pub fn build_select<E: Entity>(builder: &domain_builder::QueryBuilder<E>) -> SelectQuery {
        let columns = if builder.selected_columns.is_empty() {
            vec![Expression::Column("*".to_string())]
        } else {
            builder
                .selected_columns
                .iter()
                .map(|c| Expression::Column(c.clone()))
                .collect()
        };

        SelectQuery {
            table: E::table_name().to_string(),
            columns,
            distinct: builder.distinct,
            joins: Self::build_joins::<E>(builder),
            filters: builder
                .filters
                .iter()
                .map(|f| Self::build_condition(f))
                .collect(),
            group_by: builder
                .group_by
                .iter()
                .map(|c| Expression::Column(c.clone()))
                .collect(),
            having: builder
                .having_filters
                .iter()
                .map(|f| Self::build_condition(f))
                .collect(),
            order_by: builder
                .order_by
                .iter()
                .map(|o| Self::build_order_by(o))
                .collect(),
            limit: builder.limit,
            offset: builder.offset,
        }
    }

    pub fn build_count<E: Entity>(builder: &domain_builder::QueryBuilder<E>) -> SelectQuery {
        SelectQuery {
            table: E::table_name().to_string(),
            columns: vec![Expression::Function(
                "COUNT".to_string(),
                vec![Expression::Column("*".to_string())],
            )],
            distinct: false,
            joins: Vec::new(),
            filters: builder
                .filters
                .iter()
                .map(|f| Self::build_condition(f))
                .collect(),
            group_by: Vec::new(),
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn build_aggregate<E: Entity>(
        builder: &domain_builder::QueryBuilder<E>,
        function: &str,
        field: &str,
    ) -> SelectQuery {
        SelectQuery {
            table: E::table_name().to_string(),
            columns: vec![Expression::Function(
                function.to_string(),
                vec![Expression::Column(field.to_string())],
            )],
            distinct: false,
            joins: Vec::new(),
            filters: builder
                .filters
                .iter()
                .map(|f| Self::build_condition(f))
                .collect(),
            group_by: Vec::new(),
            having: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
        }
    }

    pub fn build_insert<E: Entity>(entity: &E) -> InsertQuery {
        let values_map = entity.to_values();
        let mut columns = Vec::new();
        let mut values = Vec::new();

        for (col, val) in values_map {
            columns.push(col);
            values.push(val);
        }

        InsertQuery {
            table: E::table_name().to_string(),
            columns,
            values,
        }
    }

    pub fn build_update<E: Entity>(entity: &E) -> UpdateQuery {
        let values_map = entity.to_values();
        let pk_name = E::primary_key();
        let mut assignments = Vec::new();
        let mut pk_value = None;

        for (col, val) in values_map {
            if col == pk_name {
                pk_value = Some(val);
                continue;
            }
            assignments.push((col, val));
        }

        let mut filters = Vec::new();
        if let Some(val) = pk_value {
            filters.push(Condition {
                left: Expression::Column(pk_name.to_string()),
                operator: domain_builder::Operator::Eq,
                right: vec![Expression::Literal(val)],
            });
        }

        UpdateQuery {
            table: E::table_name().to_string(),
            assignments,
            filters,
        }
    }

    pub fn build_delete<E: Entity>(entity: &E) -> DeleteQuery {
        let values_map = entity.to_values();
        let pk_name = E::primary_key();
        let pk_value = values_map
            .get(pk_name)
            .cloned()
            .unwrap_or(crate::domain::value_objects::Value::Null);

        DeleteQuery {
            table: E::table_name().to_string(),
            filters: vec![Condition {
                left: Expression::Column(pk_name.to_string()),
                operator: domain_builder::Operator::Eq,
                right: vec![Expression::Literal(pk_value)],
            }],
        }
    }

    fn build_joins<E: Entity>(builder: &domain_builder::QueryBuilder<E>) -> Vec<Join> {
        let mut joins = Vec::new();
        let relationships = E::relationships();
        let base_table = E::table_name();

        for rel_name in &builder.with_relations {
            if let Some(rel) = relationships.get(rel_name) {
                match rel.rel_type {
                    RelationshipType::BelongsTo => {
                        joins.push(Join {
                            table: rel.target_table.clone(),
                            join_type: JoinType::Left,
                            on: Condition {
                                left: Expression::QualifiedColumn(
                                    base_table.to_string(),
                                    rel.foreign_key.clone(),
                                ),
                                operator: domain_builder::Operator::Eq,
                                right: vec![Expression::QualifiedColumn(
                                    rel.target_table.clone(),
                                    "id".to_string(),
                                )],
                            },
                        });
                    }
                    RelationshipType::HasMany | RelationshipType::HasOne => {
                        joins.push(Join {
                            table: rel.target_table.clone(),
                            join_type: JoinType::Left,
                            on: Condition {
                                left: Expression::QualifiedColumn(
                                    rel.target_table.clone(),
                                    rel.foreign_key.clone(),
                                ),
                                operator: domain_builder::Operator::Eq,
                                right: vec![Expression::QualifiedColumn(
                                    base_table.to_string(),
                                    "id".to_string(),
                                )],
                            },
                        });
                    }
                    RelationshipType::ManyToMany => {
                        let join_table = rel.join_table.as_ref().unwrap();
                        let target_fk = rel.target_foreign_key.as_ref().unwrap();

                        joins.push(Join {
                            table: join_table.clone(),
                            join_type: JoinType::Left,
                            on: Condition {
                                left: Expression::QualifiedColumn(
                                    join_table.clone(),
                                    rel.foreign_key.clone(),
                                ),
                                operator: domain_builder::Operator::Eq,
                                right: vec![Expression::QualifiedColumn(
                                    base_table.to_string(),
                                    "id".to_string(),
                                )],
                            },
                        });

                        joins.push(Join {
                            table: rel.target_table.clone(),
                            join_type: JoinType::Left,
                            on: Condition {
                                left: Expression::QualifiedColumn(
                                    rel.target_table.clone(),
                                    "id".to_string(),
                                ),
                                operator: domain_builder::Operator::Eq,
                                right: vec![Expression::QualifiedColumn(
                                    join_table.clone(),
                                    target_fk.clone(),
                                )],
                            },
                        });
                    }
                }
            }
        }
        joins
    }

    fn build_condition(filter: &domain_builder::Condition) -> Condition {
        Condition {
            left: Expression::Column(filter.column.clone()),
            operator: filter.operator,
            right: filter
                .values
                .iter()
                .map(|v| Expression::Literal(v.clone()))
                .collect(),
        }
    }

    fn build_order_by(order: &domain_builder::OrderBy) -> OrderBy {
        OrderBy {
            expression: Expression::Column(order.column.clone()),
            direction: order.direction,
        }
    }
}
