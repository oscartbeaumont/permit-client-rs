# EnvironmentUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the environment | [optional]
**description** | Option<**String**> | an optional longer description of the environment | [optional]
**custom_branch_name** | Option<**String**> | when using gitops feature, an optional branch name for the environment | [optional]
**jwks** | Option<[**models::JwksConfig**](JwksConfig.md)> | jwks for element frontend only login | [optional]
**settings** | Option<[**serde_json::Value**](.md)> | the settings for this environment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


