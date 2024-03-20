# ResourceRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A URL-friendly name of the resource (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the resource. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the resource | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the resource belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the resource belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the resource belongs to. | 
**created_at** | **String** | Date and time when the resource was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the resource was last updated/modified (ISO_8601 format). | 
**name** | **String** | The name of the resource | 
**urn** | Option<**String**> | The [URN](https://en.wikipedia.org/wiki/Uniform_Resource_Name) (Uniform Resource Name) of the resource | [optional]
**description** | Option<**String**> | An optional longer description of what this resource respresents in your system | [optional]
**actions** | Option<[**std::collections::HashMap<String, models::ActionBlockRead>**](ActionBlockRead.md)> |          A actions definition block, typically contained within a resource type definition block.         The actions represents the ways you can interact with a protected resource.          | [optional][default to {}]
**type_attributes** | Option<[**serde_json::Value**](.md)> | optional dictionary of key-value pairs that can be used to store arbitrary metadata about this resource. This metadata can be used to filter resource using query parameters with attr_ prefix | [optional]
**attributes** | Option<[**std::collections::HashMap<String, models::AttributeBlockRead>**](AttributeBlockRead.md)> | Attributes that each resource of this type defines, and can be used in your ABAC policies. | [optional]
**roles** | Option<[**std::collections::HashMap<String, models::ResourceRoleRead>**](ResourceRoleRead.md)> | Roles defined on this resource. The key is the role name, and the value contains the role properties such as granted permissions, etc. | [optional]
**relations** | Option<[**std::collections::HashMap<String, models::RelationBlockRead>**](RelationBlockRead.md)> |          A relations definition block, typically contained within a resource type definition block.         The relations represents the ways you can interact with a protected resource.          | [optional][default to {}]
**action_groups** | Option<[**std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)> |  | [optional][default to {}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


