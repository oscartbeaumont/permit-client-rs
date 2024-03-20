# ResourceRoleUpdate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | The name of the role | [optional]
**description** | Option<**String**> | optional description string explaining what this role represents, or what permissions are granted to it. | [optional]
**permissions** | Option<**Vec<String>**> | list of action keys that define what actions this resource role is permitted to do | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | optional dictionary of key-value pairs that can be used to store arbitrary metadata about this role. This metadata can be used to filter role using query parameters with attr_ prefix, currently supports only 'equals' operator | [optional]
**extends** | Option<**Vec<String>**> | list of role keys that define what roles this role extends. In other words: this role will automatically inherit all the permissions of the given roles in this list. | [optional][default to []]
**granted_to** | Option<[**models::DerivedRoleBlockEdit**](DerivedRoleBlockEdit.md)> | Derived role that inherit will be applied on this role | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


