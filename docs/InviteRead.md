# InviteRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**member_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Unique id of the invite | [optional]
**email** | **String** | The invited member's email address | 
**role** | Option<[**models::MemberAccessLevel**](MemberAccessLevel.md)> | The role the member will be assigned with | [optional][default to Admin]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the invite | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the invite belongs to. | 
**invite_code** | [**uuid::Uuid**](uuid::Uuid.md) | The invite code that is sent to the member's email | 
**created_at** | **String** | Date and time when the invite was created (ISO_8601 format). | 
**status** | [**models::InviteStatus**](InviteStatus.md) | The status of the invite (pending, failed, etc) | 
**failed_reason** | Option<**String**> | if failed, the reason the invitation failed | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


