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

/// OnboardingStep : An enumeration.
/// An enumeration.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OnboardingStep {
    #[serde(rename = "create_organization")]
    CreateOrganization,
    #[serde(rename = "create_project")]
    CreateProject,
    #[serde(rename = "create_resource")]
    CreateResource,
    #[serde(rename = "create_actions")]
    CreateActions,
    #[serde(rename = "assign_permissions")]
    AssignPermissions,
    #[serde(rename = "assign_user_roles")]
    AssignUserRoles,
    #[serde(rename = "connect_sdk")]
    ConnectSdk,
    #[serde(rename = "done")]
    Done,

}

impl ToString for OnboardingStep {
    fn to_string(&self) -> String {
        match self {
            Self::CreateOrganization => String::from("create_organization"),
            Self::CreateProject => String::from("create_project"),
            Self::CreateResource => String::from("create_resource"),
            Self::CreateActions => String::from("create_actions"),
            Self::AssignPermissions => String::from("assign_permissions"),
            Self::AssignUserRoles => String::from("assign_user_roles"),
            Self::ConnectSdk => String::from("connect_sdk"),
            Self::Done => String::from("done"),
        }
    }
}

impl Default for OnboardingStep {
    fn default() -> OnboardingStep {
        Self::CreateOrganization
    }
}
