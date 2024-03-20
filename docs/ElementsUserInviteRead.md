# ElementsUserInviteRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the elements_user_invite | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the elements_user_invite belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the elements_user_invite belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the elements_user_invite belongs to. | 
**created_at** | **String** | Date and time when the elements_user_invite was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the elements_user_invite was last updated/modified (ISO_8601 format). | 
**key** | **String** | The key of the user that is being invited | 
**status** | [**models::UserInviteStatus**](UserInviteStatus.md) | The status of the user invite | 
**email** | **String** | The email of the user that being invited | 
**first_name** | **String** | The first name of the user that is being invited | 
**last_name** | **String** | The last name of the user that is being invited | 
**role_id** | [**uuid::Uuid**](uuid::Uuid.md) | The role of the user that is being invited | 
**tenant_id** | [**uuid::Uuid**](uuid::Uuid.md) | The tenant id of the user that is being invited | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


