# ResourceActionGroupRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | The name of the action group | 
**description** | Option<**String**> | An optional longer description of what this action group represents in your system | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | optional dictionary of key-value pairs that can be used to store arbitrary metadata about this action group. This metadata can be used to filter action groups using query parameters with attr_ prefix | [optional]
**actions** | Option<**Vec<String>**> |  | [optional][default to []]
**key** | **String** | A URL-friendly name of the action group (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the action group. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the action group | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the action group belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the action group belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the action group belongs to. | 
**resource_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the resource that the action group belongs to. | 
**created_at** | **String** | Date and time when the action group was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the action group was last updated/modified (ISO_8601 format). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


