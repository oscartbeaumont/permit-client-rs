# RoleRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the role | 
**description** | Option<**String**> | optional description string explaining what this role represents, or what permissions are granted to it. | [optional]
**permissions** | Option<**Vec<String>**> | list of action keys that define what actions this resource role is permitted to do | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | optional dictionary of key-value pairs that can be used to store arbitrary metadata about this role. This metadata can be used to filter role using query parameters with attr_ prefix, currently supports only 'equals' operator | [optional]
**extends** | Option<**Vec<String>**> | list of role keys that define what roles this role extends. In other words: this role will automatically inherit all the permissions of the given roles in this list. | [optional]
**granted_to** | Option<[**models::DerivedRoleBlockRead**](DerivedRoleBlockRead.md)> |          A derived role defintion block, typically contained whithin a role definition.         The derived role is a role that is derived from the role definition.          | [optional][default to {}]
**key** | **String** | A URL-friendly name of the role (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the role. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the role | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the role belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the role belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the role belongs to. | 
**created_at** | **String** | Date and time when the role was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the role was last updated/modified (ISO_8601 format). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


