# ResourceCreate

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A URL-friendly name of the resource (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the resource. | 
**name** | **String** | The name of the resource | 
**urn** | Option<**String**> | The [URN](https://en.wikipedia.org/wiki/Uniform_Resource_Name) (Uniform Resource Name) of the resource | [optional]
**description** | Option<**String**> | An optional longer description of what this resource respresents in your system | [optional]
**actions** | [**std::collections::HashMap<String, models::ActionBlockEditable>**](ActionBlockEditable.md) |          A actions definition block, typically contained within a resource type definition block.         The actions represents the ways you can interact with a protected resource.          | 
**type_attributes** | Option<[**serde_json::Value**](.md)> | optional dictionary of key-value pairs that can be used to store arbitrary metadata about this resource. This metadata can be used to filter resource using query parameters with attr_ prefix | [optional]
**attributes** | Option<[**std::collections::HashMap<String, models::AttributeBlockEditable>**](AttributeBlockEditable.md)> | Attributes that each resource of this type defines, and can be used in your ABAC policies. | [optional]
**roles** | Option<[**std::collections::HashMap<String, models::RoleBlockEditable>**](RoleBlockEditable.md)> | Roles defined on this resource. The key is the role name, and the value contains the role properties such as granted permissions, base roles, etc. | [optional]
**relations** | Option<**std::collections::HashMap<String, String>**> | Relations to other resources. The key is the relation key, and the value is the related resource. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


