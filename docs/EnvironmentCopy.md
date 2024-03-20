# EnvironmentCopy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**target_env** | [**models::EnvironmentCopyTarget**](EnvironmentCopyTarget.md) | If copying a new environment, the environment configuration. If copying to an existing environment, the environment identifier | 
**conflict_strategy** | Option<**String**> | Action to take when detecting a conflict when copying. Only applies to copying into an existing environment | [optional][default to Fail]
**scope** | Option<[**models::EnvironmentCopyScope**](EnvironmentCopyScope.md)> | Filters to include and exclude copied objects | [optional][default to {resources={include=[], exclude=[]}, roles={include=[], exclude=[]}, user_sets={include=[], exclude=[]}, resource_sets={include=[], exclude=[]}}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


