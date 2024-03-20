# RoleAssignmentDetailedRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the role assignment | 
**role** | [**models::RoleAssignmentRole**](RoleAssignmentRole.md) | the role that is assigned | 
**user** | [**models::RoleAssignmentUser**](RoleAssignmentUser.md) | the user the role is assigned to | 
**tenant** | [**models::RoleAssignmentTenant**](RoleAssignmentTenant.md) | the tenant the role is associated with | 
**resource_instance** | Option<[**models::RoleAssignmentResourceInstance**](RoleAssignmentResourceInstance.md)> |  | [optional]
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the role assignment belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the role assignment belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the role assignment belongs to. | 
**created_at** | **String** | Date and time when the role assignment was created (ISO_8601 format). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


