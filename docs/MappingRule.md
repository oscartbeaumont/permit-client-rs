# MappingRule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**url** | **String** | The URL to match against the request URL | 
**http_method** | [**models::Methods**](Methods.md) | The HTTP method to match against the request method | 
**resource** | **String** | The resource to match against the request resource | 
**headers** | Option<**std::collections::HashMap<String, String>**> | The headers to match against the request headers | [optional][default to {}]
**action** | Option<**String**> | The action to match against the request action | [optional]
**priority** | Option<**i32**> | The priority of the mapping rule. The higher the priority, the higher the precedence | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


