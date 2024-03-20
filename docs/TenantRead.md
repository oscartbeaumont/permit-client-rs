# TenantRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A unique id by which Permit will identify the tenant. The tenant key must be url-friendly (slugified). | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the tenant | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the tenant belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the tenant belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the tenant belongs to. | 
**created_at** | **String** | Date and time when the tenant was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the tenant was last updated/modified (ISO_8601 format). | 
**last_action_at** | **String** | Date and time when the tenant was last active (ISO_8601 format). In other words, this is the last time a permission check was done on a resource belonging to this tenant. | 
**name** | **String** | A descriptive name for the tenant | 
**description** | Option<**String**> | an optional longer description of the tenant | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | Arbitraty tenant attributes that will be used to enforce attribute-based access control policies. | [optional][default to {}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


