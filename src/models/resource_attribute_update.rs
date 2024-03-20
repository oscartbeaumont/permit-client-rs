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
pub struct ResourceAttributeUpdate {
    /// The type of the attribute, we currently support: `bool`, `number` (ints, floats), `time` (a timestamp), `string`, and `json`.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<models::AttributeType>,
    /// An optional longer description of what this attribute respresents in your system
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl ResourceAttributeUpdate {
    pub fn new() -> ResourceAttributeUpdate {
        ResourceAttributeUpdate {
            r#type: None,
            description: None,
        }
    }
}
