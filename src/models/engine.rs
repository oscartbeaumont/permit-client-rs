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

/// Engine : An enumeration.
/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Engine {
    #[serde(rename = "OPA")]
    Opa,
    #[serde(rename = "AVP")]
    Avp,

}

impl ToString for Engine {
    fn to_string(&self) -> String {
        match self {
            Self::Opa => String::from("OPA"),
            Self::Avp => String::from("AVP"),
        }
    }
}

impl Default for Engine {
    fn default() -> Engine {
        Self::Opa
    }
}
