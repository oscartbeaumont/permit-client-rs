# RelationRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | An optional longer description of what this relation represents in your system | [optional]
**subject_resource** | **String** | The subject resource ID or key | 
**key** | **String** | A URL-friendly name of the relation (i.e: slug). You will be able to query later using this key instead of the id (UUID) of the relation. | 
**name** | **String** | The name of the relation | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the relation | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the relation belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the relation belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the relation belongs to. | 
**created_at** | **String** | Date and time when the relation was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the relation was last updated/modified (ISO_8601 format). | 
**object_resource_id** | [**uuid::Uuid**](uuid::Uuid.md) | The object resource id | 
**object_resource** | **String** | The object resource key | 
**subject_resource_id** | [**uuid::Uuid**](uuid::Uuid.md) | The subject resource id | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


