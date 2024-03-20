# SmtpEmailConfigurationRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**host** | **String** | The host of the SMTP provider | 
**from_address** | **String** | The from address the mails will be sent from | 
**port** | **i32** | The port of the SMTP provider | 
**username** | **String** | The username of the SMTP provider | 
**password** | **String** | The password of the SMTP provider | 
**email_provider_type** | Option<**String**> | The type of the email provider | [optional][default to Smtp]
**id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the email_configuration | 
**organization_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the organization that the email_configuration belongs to. | 
**project_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the project that the email_configuration belongs to. | 
**environment_id** | [**uuid::Uuid**](uuid::Uuid.md) | Unique id of the environment that the email_configuration belongs to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


