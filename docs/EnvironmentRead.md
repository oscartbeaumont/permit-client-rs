# EnvironmentRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A URL-friendly name of the environment (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the environment. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the environment belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the environment belongs to. | 
**created_at** | **String** | Date and time when the environment was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the environment was last updated/modified (ISO_8601 format). | 
**name** | **String** | The name of the environment | 
**description** | Option<**String**> | an optional longer description of the environment | [optional]
**custom_branch_name** | Option<**String**> | when using gitops feature, an optional branch name for the environment | [optional]
**jwks** | Option<[**models::JwksConfig**](JwksConfig.md)> | jwks for element frontend only login | [optional]
**settings** | Option<[**serde_json::Value**](.md)> | the settings for this environment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


