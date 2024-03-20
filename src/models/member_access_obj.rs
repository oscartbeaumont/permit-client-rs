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

/// MemberAccessObj : An enumeration.
/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MemberAccessObj {
    #[serde(rename = "org")]
    Org,
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "env")]
    Env,

}

impl ToString for MemberAccessObj {
    fn to_string(&self) -> String {
        match self {
            Self::Org => String::from("org"),
            Self::Project => String::from("project"),
            Self::Env => String::from("env"),
        }
    }
}

impl Default for MemberAccessObj {
    fn default() -> MemberAccessObj {
        Self::Org
    }
}
