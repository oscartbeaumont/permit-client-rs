# RelationshipTupleRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**subject** | **String** | resource_key:resource_instance_key of the subject | 
**relation** | **String** | key of the assigned relation | 
**object** | **String** | resource_key:resource_instance_key of the object | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the relationship tuple | 
**tenant** | **String** | The tenant the relationship tuple is associated with | 
**subject_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the subject | 
**relation_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the relation | 
**object_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the object | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the tenant | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the relationship tuple belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the relationship tuple belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the relationship tuple belongs to. | 
**created_at** | **String** | Date and time when the relationship tuple was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the relationship tuple was created (ISO_8601 format). | 
**subject_details** | Option<[**models::ResourceInstanceBlockRead**](ResourceInstanceBlockRead.md)> | The subject details of the relationship tuple | [optional]
**relation_details** | Option<[**models::StrippedRelationBlockRead**](StrippedRelationBlockRead.md)> | The relation details of the relationship tuple | [optional]
**object_details** | Option<[**models::ResourceInstanceBlockRead**](ResourceInstanceBlockRead.md)> | The object details of the relationship tuple | [optional]
**tenant_details** | Option<[**models::TenantBlockRead**](TenantBlockRead.md)> | The tenant details of the relationship tuple | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


