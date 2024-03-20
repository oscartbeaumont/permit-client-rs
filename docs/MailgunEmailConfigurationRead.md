# MailgunEmailConfigurationRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**from_address** | **String** | The from address the mails will be sent from | 
**api_key** | **String** | The api key of the mail provider | 
**region** | **String** | The region of the mail provider | 
**domain** | **String** | The domain of the mail provider | 
**email_provider_type** | Option<**String**> | The type of the email provider | [optional][default to Mailgun]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the email_configuration | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the email_configuration belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the email_configuration belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the email_configuration belongs to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


