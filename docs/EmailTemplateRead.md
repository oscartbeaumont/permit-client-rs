# EmailTemplateRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**template_type** | [**models::EmailTemplateType**](EmailTemplateType.md) | The type of the email template, can be either 'approval_flows' or 'user_management' | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the EmailTemplate | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the EmailTemplate belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the EmailTemplate belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the EmailTemplate belongs to. | 
**from_address** | **String** | The from address the mails will be sent from | 
**redirect_to** | **String** | The redirect url the user will be redirected to after clicking the link in the email | 
**url_ttl** | **String** | The time to live of the url in the email, in seconds | 
**subject** | **String** | The subject of the email template | 
**messages** | [**Vec<models::EmailTemplateMessage>**](EmailTemplateMessage.md) | The messages of the email template | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


