# ResourceActionGroupCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A URL-friendly name of the action group (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the action group. | 
**name** | **String** | The name of the action group | 
**description** | Option<**String**> | An optional longer description of what this action group represents in your system | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | optional dictionary of key-value pairs that can be used to store arbitrary metadata about this action group. This metadata can be used to filter action groups using query parameters with attr_ prefix | [optional]
**actions** | Option<**Vec<String>**> |  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


