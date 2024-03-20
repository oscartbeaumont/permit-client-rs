/*
 * Permit.io API
 *
 *  Authorization as a service 
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeBlockEditable {
    /// The type of the attribute, we currently support: `bool`, `number` (ints, floats), `time` (a timestamp), `string`, and `json`.
    #[serde(rename = "type")]
    pub r#type: models::AttributeType,
    /// optional description string explaining what data this attribute will store
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl AttributeBlockEditable {
    pub fn new(r#type: models::AttributeType) -> AttributeBlockEditable {
        AttributeBlockEditable {
            r#type,
            description: None,
        }
    }
}
