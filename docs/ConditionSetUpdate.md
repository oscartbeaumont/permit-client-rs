# ConditionSetUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | A descriptive name for the set, i.e: 'US based employees' or 'Users behind VPN' | [optional]
**description** | Option<**String**> | an optional longer description of the set | [optional]
**conditions** | Option<[**serde_json::Value**](.md)> | a boolean expression that consists of multiple conditions, with and/or logic. | [optional][default to {}]
**parent_id** | Option<[**models::ParentId**](Parent_Id.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


