# ProjectCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A URL-friendly name of the project (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the project. | 
**urn_namespace** | Option<**String**> | Optional namespace for URNs. If empty, URNs will be generated from project key. | [optional]
**name** | **String** | The name of the project | 
**description** | Option<**String**> | a longer description outlining the project objectives | [optional]
**settings** | Option<[**serde_json::Value**](.md)> | the settings for this project | [optional]
**active_policy_repo_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | the id of the policy repo to use for this project | [optional]
**initial_environments** | Option<[**Vec<models::EnvironmentCreate>**](EnvironmentCreate.md)> | The initial environments to create for this project. By default, 'Development' and 'Production' are created, specify [] (empty list) to skip that. | [optional][default to [{key=dev, name=Development}, {key=production, name=Production}]]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


