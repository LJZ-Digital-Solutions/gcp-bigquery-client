use std::collections::HashSet;

use crate::model::field_type::FieldType;
use crate::model::table_field_schema_categories::TableFieldSchemaCategories;
use crate::model::table_field_schema_policy::TableFieldSchemaPolicyTags;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TableFieldSchema {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<TableFieldSchemaCategories>,
    /// [Optional] The field description. The maximum length is 1,024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// [Optional] Describes the nested schema fields if the type property is set to RECORD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TableFieldSchema>>,
    /// [Optional] The field mode. Possible values include NULLABLE, REQUIRED and REPEATED. The default value is NULLABLE.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// [Required] The field name. The name must contain only letters (a-z, A-Z), numbers (0-9), or underscores (_), and must start with a letter or underscore. The maximum length is 128 characters.
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_tags: Option<TableFieldSchemaPolicyTags>,
    /// [Required] The field data type. Possible values include STRING, BYTES, INTEGER, INT64 (same as INTEGER), FLOAT, FLOAT64 (same as FLOAT), NUMERIC, BIGNUMERIC, BOOLEAN, BOOL (same as BOOLEAN), TIMESTAMP, DATE, TIME, DATETIME, RECORD (where RECORD indicates that the field contains a nested schema) or STRUCT (same as RECORD).
    pub r#type: FieldType,
}

impl Eq for TableFieldSchema {}

impl PartialEq for TableFieldSchema {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.mode == other.mode
            && self.r#type == other.r#type
            && match (&self.fields, &other.fields) {
                (Some(a), Some(b)) => {
                    let set_fields_a: HashSet<&TableFieldSchema> = a.iter().collect();
                    let set_fields_b: HashSet<&TableFieldSchema> = b.iter().collect();
                    set_fields_a == set_fields_b
                }
                (None, None) => true,
                _ => false,
            }
    }
}

impl std::hash::Hash for TableFieldSchema {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.mode.hash(state);
        self.r#type.hash(state);

        // For fields, we need to handle the Vec in a way that matches PartialEq
        // If PartialEq treats it as a set, Hash should too
        if let Some(fields) = &self.fields {
            // Sort fields to get consistent hashing regardless of order
            let mut sorted_fields: Vec<_> = fields.iter().collect();
            sorted_fields.sort_by_key(|f| {
                // Create a stable sort key
                (&f.name, &f.r#type)
            });
            sorted_fields.hash(state);
        } else {
            None::<Vec<&TableFieldSchema>>.hash(state);
        }
    }
}

impl TableFieldSchema {
    pub fn new(field_name: &str, field_type: FieldType) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: field_type,
        }
    }

    pub fn integer(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Integer,
        }
    }

    pub fn float(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Float,
        }
    }

    pub fn bool(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Bool,
        }
    }

    pub fn string(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::String,
        }
    }

    pub fn record(field_name: &str, fields: Vec<TableFieldSchema>) -> Self {
        Self {
            categories: None,
            description: None,
            fields: Some(fields),
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Record,
        }
    }

    pub fn bytes(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Bytes,
        }
    }

    pub fn numeric(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Numeric,
        }
    }

    pub fn big_numeric(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Bignumeric,
        }
    }

    pub fn timestamp(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Timestamp,
        }
    }

    pub fn date(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Date,
        }
    }

    pub fn time(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Time,
        }
    }

    pub fn date_time(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Datetime,
        }
    }

    pub fn geography(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Geography,
        }
    }

    pub fn json(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Json,
        }
    }

    pub fn interval(field_name: &str) -> Self {
        Self {
            categories: None,
            description: None,
            fields: None,
            mode: None,
            name: field_name.into(),
            policy_tags: None,
            r#type: FieldType::Interval,
        }
    }
}
