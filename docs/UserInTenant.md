# UserInTenant

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**tenant** | **String** | The tenant key which the user is associated with | 
**roles** | **Vec<String>** | List of roles assigned to the user in that tenant | 
**status** | [**models::UserStatus**](UserStatus.md) | Whether the user has signed in or not | 
**resource_instance_roles** | Option<[**Vec<models::UserResourceInstanceRole>**](UserResourceInstanceRole.md)> |  | [optional][default to []]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


