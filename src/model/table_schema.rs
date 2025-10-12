use std::collections::HashSet;

use crate::model::table_field_schema::TableFieldSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TableSchema {
    /// Describes the fields in a table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TableFieldSchema>>,
}

/// `TableSchemas` contain lists of nested structures. This equality check does not take the order
/// of elements into account, it rather checks for set equality.
impl PartialEq for TableSchema {
    fn eq(&self, other: &Self) -> bool {
        match (&self.fields, &other.fields) {
            (Some(a), Some(b)) => {
                let set_a: HashSet<&TableFieldSchema> = a.iter().collect();
                let set_b: HashSet<&TableFieldSchema> = b.iter().collect();
                set_a == set_b
            }
            (None, None) => true,
            _ => false,
        }
    }
}

impl TableSchema {
    pub fn new(fields: Vec<TableFieldSchema>) -> Self {
        Self { fields: Some(fields) }
    }

    pub fn fields(&self) -> &Option<Vec<TableFieldSchema>> {
        &self.fields
    }

    pub fn field_count(&self) -> usize {
        self.fields.as_ref().map_or(0, |fields| fields.len())
    }

    pub fn is_none(&self) -> bool {
        self.fields.is_none()
    }
}

#[cfg(test)]
mod test {
    use crate::model::{
        table_field_schema::TableFieldSchema, table_field_schema_categories::TableFieldSchemaCategories,
        table_field_schema_policy::TableFieldSchemaPolicyTags, table_schema::TableSchema,
    };

    #[tokio::test]
    async fn equality_simple() {
        let a = TableSchema::new(vec![TableFieldSchema::integer("int_value")]);
        let b = TableSchema::new(vec![TableFieldSchema::integer("int_value")]);
        assert!(a == b);
    }

    #[tokio::test]
    async fn inequality_simple() {
        let a = TableSchema::new(vec![TableFieldSchema::integer("int_value")]);
        let b = TableSchema::new(vec![TableFieldSchema::integer("different")]);
        assert!(a != b);
    }

    #[tokio::test]
    async fn equality() {
        let a = TableSchema::new(vec![
            TableFieldSchema::integer("int_value"),
            TableFieldSchema::float("float_value"),
            TableFieldSchema::bool("bool_value"),
            TableFieldSchema::string("string_value"),
            TableFieldSchema::record(
                "record_value",
                vec![
                    TableFieldSchema::integer("int_value"),
                    TableFieldSchema::string("string_value"),
                    TableFieldSchema::record(
                        "record_value",
                        vec![
                            TableFieldSchema::integer("int_value"),
                            TableFieldSchema::string("string_value"),
                        ],
                    ),
                ],
            ),
            TableFieldSchema::json("json_value"),
        ]);

        let b = TableSchema::new(vec![
            TableFieldSchema::integer("int_value"),
            TableFieldSchema::float("float_value"),
            TableFieldSchema::bool("bool_value"),
            TableFieldSchema::string("string_value"),
            TableFieldSchema::record(
                "record_value",
                vec![
                    TableFieldSchema::integer("int_value"),
                    TableFieldSchema::string("string_value"),
                    TableFieldSchema::record(
                        "record_value",
                        vec![
                            TableFieldSchema::integer("int_value"),
                            TableFieldSchema::string("string_value"),
                        ],
                    ),
                ],
            ),
            TableFieldSchema::json("json_value"),
        ]);

        assert!(a == b);
    }

    #[tokio::test]
    async fn inequality() {
        let a = TableSchema::new(vec![
            TableFieldSchema::integer("int_value"),
            TableFieldSchema::float("float_value"),
            TableFieldSchema::bool("bool_value"),
            TableFieldSchema::string("string_value"),
            TableFieldSchema::record(
                "record_value",
                vec![
                    TableFieldSchema::integer("int_value"),
                    TableFieldSchema::string("string_value"),
                    TableFieldSchema::record(
                        "record_value",
                        vec![
                            TableFieldSchema::integer("int_value"),
                            TableFieldSchema::string("other_string"),
                        ],
                    ),
                ],
            ),
            TableFieldSchema::json("json_value"),
        ]);

        let b = TableSchema::new(vec![
            TableFieldSchema::integer("int_value"),
            TableFieldSchema::float("float_value"),
            TableFieldSchema::bool("bool_value"),
            TableFieldSchema::string("string_value"),
            TableFieldSchema::record(
                "record_value",
                vec![
                    TableFieldSchema::integer("int_value"),
                    TableFieldSchema::string("string_value"),
                    TableFieldSchema::record(
                        "record_value",
                        vec![
                            TableFieldSchema::integer("int_value"),
                            TableFieldSchema::string("string_value"),
                        ],
                    ),
                ],
            ),
            TableFieldSchema::json("json_value"),
        ]);

        assert!(a != b);
    }

    #[tokio::test]
    async fn unordered() {
        let a = TableSchema::new(vec![TableFieldSchema::record(
            "record_value",
            vec![
                TableFieldSchema::integer("int_value"),
                TableFieldSchema::string("string_value"),
            ],
        )]);

        let b = TableSchema::new(vec![TableFieldSchema::record(
            "record_value",
            vec![
                TableFieldSchema::string("string_value"),
                TableFieldSchema::integer("int_value"),
            ],
        )]);

        assert!(a == b);
    }

    #[tokio::test]
    async fn identical_ordered() {
        let some_schema = TableFieldSchema::new("some_schema", crate::model::field_type::FieldType::String);
        let another_schema = TableFieldSchema::new("another_schema", crate::model::field_type::FieldType::String);

        let a = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: Some(vec![some_schema.clone(), another_schema.clone()]),
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };
        let b = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: Some(vec![some_schema, another_schema]),
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };

        assert!(a == b)
    }

    #[tokio::test]
    async fn identical_unordered() {
        let some_schema = TableFieldSchema::new("some_schema", crate::model::field_type::FieldType::String);
        let another_schema = TableFieldSchema::new("another_schema", crate::model::field_type::FieldType::String);

        let a = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: Some(vec![some_schema.clone(), another_schema.clone()]),
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };
        let b = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: Some(vec![another_schema, some_schema]),
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };

        assert!(a == b)
    }

    /// Categories do not discriminate whether TableSchemas are different
    #[tokio::test]
    async fn different_categories() {
        let a = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: Some(TableFieldSchemaCategories { names: None }),
                description: None,
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };
        let b = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: Some(TableFieldSchemaCategories {
                    names: Some(vec!["name".to_string()]),
                }),
                description: None,
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };

        assert!(a == b)
    }

    /// Descriptions do not discriminate whether TableSchemas are different
    #[tokio::test]
    async fn different_descriptions() {
        let a = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };
        let b = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: Some("some_description".to_string()),
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };

        assert!(a == b)
    }

    /// PolicyTags do not discriminate whether TableSchemas are different
    #[tokio::test]
    async fn different_policy_tags() {
        let a = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: Some(TableFieldSchemaPolicyTags {
                    names: Some(vec!["tag".to_string()]),
                }),
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };
        let b = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };

        assert!(a == b)
    }

    /// Mode does discrimate different TableSchemas
    #[tokio::test]
    async fn different_modes() {
        let a = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: Some("mode".to_string()),
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };
        let b = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };

        assert!(a != b)
    }

    /// Name does discrimate different TableSchemas
    #[tokio::test]
    async fn different_names() {
        let a = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: None,
                name: "not the same!".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };
        let b = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };

        assert!(a != b)
    }

    /// Type does discrimate different TableSchemas
    #[tokio::test]
    async fn different_types() {
        let a = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: None,
                name: "not the same!".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::Bool,
            }]),
        };
        let b = TableSchema {
            fields: Some(vec![TableFieldSchema {
                categories: None,
                description: None,
                fields: None,
                mode: None,
                name: "same".to_string(),
                policy_tags: None,
                r#type: crate::model::field_type::FieldType::String,
            }]),
        };

        assert!(a != b)
    }
}
