# IdentityRead

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_id** | **String** | Unique User Id of this identity in the identity provider (including the provider type) | 
**provider** | **String** | The identity provider type this identity came from | 
**sub** | **String** | Unique User Id of this identity in the identity provider (NOT including the provider type) | 
**email** | **String** | Email connected to this account identity | 
**email_verified** | **bool** | Whether this email address connected to this account identity is verified or not. For social providers like 'Login with Google' this is done automatically, otherwise we will send the user a verification link in email. | 
**auth0_info** | [**serde_json::Value**](.md) | Raw user info json coming from our identity provider and matching a specific account identity | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


