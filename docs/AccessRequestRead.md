# AccessRequestRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**access_request_details** | [**models::AccessRequestDetails**](AccessRequestDetails.md) | details of the access request, including the resource and tenant | 
**reason** | Option<**String**> | Optional business justification provided by the user requesting access | [optional]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the access request | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the access request belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the access request belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the access request belongs to. | 
**created_at** | **String** | Date and time when the access request was created (ISO_8601 format). | 
**updated_at** | **String** | Date and time when the access request was last updated/modified (ISO_8601 format). | 
**requesting_user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | optional id of the user that is requesting the access | [optional]
**reviewed_at** | Option<**String**> | when the access request was reviewed | [optional]
**reviewer_comment** | Option<**String**> | comment provided by the reviewer_user_id | [optional]
**status** | [**models::RequestStatus**](RequestStatus.md) | current status of the access request | 
**reviewer_user_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Optional id of the user who review the access request | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


