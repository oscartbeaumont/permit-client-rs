# RoleAssignmentRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the role assignment | 
**user** | **String** | the user the role is assigned to | 
**role** | **String** | the role that is assigned | 
**tenant** | Option<**String**> | the tenant the role is associated with | [optional]
**resource_instance** | Option<**String**> | the resource instance the role is associated with | [optional]
**resource_instance_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique id of the resource instance | [optional]
**user_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the user | 
**role_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the role | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the tenant | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the role assignment belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the role assignment belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the role assignment belongs to. | 
**created_at** | **String** | Date and time when the role assignment was created (ISO_8601 format). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


