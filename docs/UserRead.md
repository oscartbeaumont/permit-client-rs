# UserRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**key** | **String** | A unique id by which Permit will identify the user for permission checks. | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the user | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the user belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the user belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the user belongs to. | 
**associated_tenants** | Option<[**Vec<models::UserInTenant>**](UserInTenant.md)> |  | [optional][default to []]
**roles** | Option<[**Vec<models::UserRole>**](UserRole.md)> |  | [optional][default to []]
**email** | Option<**String**> | The email of the user. If synced, will be unique inside the environment. | [optional]
**first_name** | Option<**String**> | First name of the user. | [optional]
**last_name** | Option<**String**> | Last name of the user. | [optional]
**attributes** | Option<[**serde_json::Value**](.md)> | Arbitrary user attributes that will be used to enforce attribute-based access control policies. | [optional][default to {}]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


