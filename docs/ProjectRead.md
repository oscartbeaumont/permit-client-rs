# ProjectRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A URL-friendly name of the project (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the project. | 
**urn_namespace** | Option<**String**> | Optional namespace for URNs. If empty, URNs will be generated from project key. | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the project belongs to. | 
**created_at** | **String** | Date and time when the project was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the project was last updated/modified (ISO_8601 format). | 
**name** | **String** | The name of the project | 
**description** | Option<**String**> | a longer description outlining the project objectives | [optional]
**settings** | Option<[**serde_json::Value**](.md)> | the settings for this project | [optional]
**active_policy_repo_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | the id of the policy repo to use for this project | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


