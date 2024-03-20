# ResourceAttributeRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**r#type** | [**models::AttributeType**](AttributeType.md) | The type of the attribute, we currently support: `bool`, `number` (ints, floats), `time` (a timestamp), `string`, and `json`. | 
**description** | Option<**String**> | An optional longer description of what this attribute respresents in your system | [optional]
**key** | **String** | A URL-friendly name of the attribute (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the attribute. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the attribute | 
**resource_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the resource that the attribute belongs to. | 
**resource_key** | **String** | A URL-friendly name of the resource (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the resource. | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the attribute belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the attribute belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the attribute belongs to. | 
**created_at** | **String** | Date and time when the attribute was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the attribute was last updated/modified (ISO_8601 format). | 
**built_in** | **bool** | Whether the attribute is built-in, and managed by the Permit system. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


